use crate::config::config::Config;
use crate::error::http_response_error::{MapHttpResponseError, MapToBasicResult};
use crate::util::types::WebResult;
use keycloak::types::{
    ClientRepresentation, RealmRepresentation, RoleRepresentation, ServerInfoRepresentation,
    UserRepresentation,
};
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use log::debug;
use reqwest::Client;
use serde::Deserialize;
use shared::util::types::BasicResult;
use std::cmp::min;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
struct KeycloakCertificate {
    #[serde(rename = "use")]
    pub use_: Option<String>,
    #[serde(rename = "x5c")]
    pub certificates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
struct KeycloakCertificates {
    pub keys: Option<Vec<KeycloakCertificate>>,
}

#[derive(Clone)]
pub struct KeycloakService {
    admin: Arc<KeycloakAdmin>,
    realm: String,
    client: Client,
    url: String,
}

impl KeycloakService {
    pub async fn new(config: &Config) -> BasicResult<Self> {
        let client = Client::new();
        let token = KeycloakAdminToken::acquire(
            &config.keycloak_url,
            &config.keycloak_user,
            &config.keycloak_password,
            &client,
        )
        .await?;

        let admin = KeycloakAdmin::new(&config.keycloak_url, token, client.clone());

        Ok(Self {
            admin: Arc::new(admin),
            realm: config.keycloak_realm.clone(),
            client,
            url: config.keycloak_url.clone(),
        })
    }

    pub async fn init_realm(&self) -> BasicResult<()> {
        if self.get_realm(self.realm.as_str()).await.is_err() {
            debug!("Creating realm {}", self.realm);
            self.create_realm(RealmRepresentation {
                realm: Some(self.realm.clone()),
                enabled: Some(true),
                ..Default::default()
            })
            .await?;
        }

        if self
            .get_client_by_name("ca-backend")
            .await
            .map_error_to_basic()?
            .is_empty()
        {
            debug!("Creating client ca-backend");
            self.create_client(ClientRepresentation {
                client_id: Some("ca-backend".to_string()),
                enabled: Some(true),
                client_authenticator_type: Some("client-secret".to_string()),
                redirect_uris: Some(vec!["http://localhost*".into()]),
                web_origins: Some(vec!["*".into()]),
                consent_required: Some(false),
                standard_flow_enabled: Some(true),
                implicit_flow_enabled: Some(true),
                direct_access_grants_enabled: Some(true),
                service_accounts_enabled: Some(false),
                public_client: Some(true),
                frontchannel_logout: Some(true),
                protocol: Some("openid-connect".to_string()),
                attributes: Some(
                    [
                        ("oidc.ciba.grant.enabled", "false"),
                        ("post.logout.redirect.uris", "http://localhost*"),
                        ("oauth2.device.authorization.grant.enabled", "true"),
                        ("backchannel.logout.session.required", "true"),
                        ("backchannel.logout.revoke.offline.tokens", "false"),
                    ]
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string().into()))
                    .collect(),
                ),
                ..Default::default()
            })
            .await
            .map_error_to_basic()?;
        }

        self.check_and_create_role("admin").await?;

        Ok(())
    }

    async fn check_and_create_role(&self, name: &str) -> BasicResult<()> {
        if self.get_role_by_name(name).await.is_err() {
            debug!("Creating role {}", name);
            self.create_role(RoleRepresentation {
                name: Some(name.to_string()),
                ..Default::default()
            })
            .await
            .map_error_to_basic()?;
        }

        Ok(())
    }

    async fn get_realm(&self, realm: &str) -> BasicResult<RealmRepresentation> {
        self.admin.realm_get(realm).await.map_err(|e| e.into())
    }

    async fn create_realm(&self, realm: RealmRepresentation) -> BasicResult<()> {
        self.admin.post(realm).await.map_err(|e| e.into())
    }

    pub async fn get_server_info(&self) -> WebResult<ServerInfoRepresentation> {
        self.admin
            .get()
            .await
            .map_failed_dependency(Some("Failed to get server info"))
    }

    pub async fn get_realm_public_key(&self) -> BasicResult<String> {
        debug!(
            "{:?}",
            self.client
                .get(format!(
                    "{}/realms/{}/protocol/openid-connect/certs",
                    self.url, self.realm
                ))
                .send()
                .await?
                .json::<KeycloakCertificates>()
                .await?
        );

        let key = self
            .client
            .get(format!(
                "{}/realms/{}/protocol/openid-connect/certs",
                self.url, self.realm
            ))
            .send()
            .await?
            .error_for_status()?
            .json::<KeycloakCertificates>()
            .await?
            .keys
            .ok_or("No keys found in keycloak realm".to_string())?
            .into_iter()
            .find(|k| k.use_.is_some() && k.use_.as_ref().unwrap() == "sig")
            .ok_or("No signing key found in keycloak realm".to_string())?
            .certificates
            .ok_or("No public key found in keycloak realm".to_string())?
            .first()
            .ok_or("No public key found in keycloak realm".to_string())?
            .clone();

        /*let key = certificates.first()
        .ok_or("No public key found in keycloak realm".to_string())?;*/

        /*let key = self
        .admin
        .realm_keys_get(self.realm.as_str())
        .await
        .map_err(|e| e.to_string())?
        .keys
        .ok_or("No keys found in keycloak realm".to_string())?
        .into_iter()
        .find(|k| k.use_ == Some(KeysMetadataRepresentationKeyMetadataRepresentationUse::Sig))
        .ok_or("No signing key found in keycloak realm".to_string())?
        .public_key
        .ok_or("No public key found in keycloak realm".to_string())?;*/

        let mut lines = vec!["-----BEGIN PUBLIC KEY-----"];
        for i in (0..key.len()).step_by(64) {
            lines.push(&key[i..min(i + 64, key.len())]);
        }
        lines.push("-----END PUBLIC KEY-----");

        Ok(lines.join("\n"))
    }

    pub async fn get_users(
        &self,
        username: String,
        exact: bool,
    ) -> WebResult<Vec<UserRepresentation>> {
        self.admin
            .realm_users_get(
                self.realm.as_str(),
                None,
                None,
                None,
                None,
                Some(exact),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(username),
            )
            .await
            .map_failed_dependency(Some("Failed to find matching users in keycloak"))
    }

    pub async fn create_user(&self, user: UserRepresentation) -> WebResult<()> {
        self.admin
            .realm_users_post(self.realm.as_str(), user)
            .await
            .map_failed_dependency(Some("Failed to create the user in keycloak"))
    }

    pub async fn get_user_by_id(&self, id: &String) -> WebResult<UserRepresentation> {
        self.admin
            .realm_users_with_id_get(self.realm.as_str(), id.as_str())
            .await
            .map_internal_error(Some("Keycloak user not found"))
    }

    pub async fn delete_user(&self, id: &String) -> WebResult<()> {
        self.admin
            .realm_users_with_id_delete(self.realm.as_str(), id.as_str())
            .await
            .map_failed_dependency(Some("Failed to delete the user in keycloak"))
    }

    pub async fn get_client_by_name(&self, id: &str) -> WebResult<Vec<ClientRepresentation>> {
        self.admin
            .realm_clients_get(
                self.realm.as_str(),
                Some(id.to_string()),
                None,
                None,
                None,
                None,
                None,
            )
            .await
            .map_failed_dependency(Some("Failed to find matching client in keycloak"))
    }

    pub async fn create_client(&self, client: ClientRepresentation) -> WebResult<()> {
        self.admin
            .realm_clients_post(self.realm.as_str(), client)
            .await
            .map_failed_dependency(Some("Failed to create the client in keycloak"))
    }

    async fn get_role_by_name(&self, role_name: &str) -> WebResult<RoleRepresentation> {
        self.admin
            .realm_roles_with_role_name_get(self.realm.as_str(), role_name)
            .await
            .map_failed_dependency(Some("Failed to find matching role in keycloak"))
    }

    async fn create_role(&self, role: RoleRepresentation) -> WebResult<()> {
        self.admin
            .realm_roles_post(self.realm.as_str(), role)
            .await
            .map_failed_dependency(Some("Failed to create the role in keycloak"))
    }

    /*pub async fn update_user(&self, id: &String, user: UserRepresentation) -> WebResult<()> {
        self.admin
            .realm_users_with_id_put(self.realm.as_str(), id.as_str(), user)
            .await
            .map_failed_dependency(Some("Failed to update the user in keycloak"))
    }*/
}
