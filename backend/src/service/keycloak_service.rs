use crate::config::config::Config;
use crate::entity::user;
use crate::error::http_response_error::{
    HttpResponseError, MapHttpResponseError, MapKeycloakError, MapToBasicResult,
};
use crate::service::user_service::UserService;
use crate::util::types::WebResult;
use async_trait::async_trait;
use futures_util::future::join_all;
use keycloak::types::{
    ClientRepresentation, RealmRepresentation, RoleRepresentation, ServerInfoRepresentation,
    UserRepresentation,
};
use keycloak::{KeycloakAdmin, KeycloakAdminToken, KeycloakError, KeycloakTokenSupplier};
use log::{debug, info};
use reqwest::Client;
use sea_orm::ActiveValue;
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
    admin: Arc<KeycloakAdmin<KeycloakAdminTokenRetriever>>,
    realm: String,
    client: Client,
    url: String,
    admin_name: String,
    admin_password: String,
}

struct KeycloakAdminTokenRetriever {
    url: String,
    username: String,
    password: String,
    client: Client,
}

impl KeycloakAdminTokenRetriever {
    pub fn new(url: String, username: String, password: String) -> Self {
        Self {
            url,
            username,
            password,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl KeycloakTokenSupplier for KeycloakAdminTokenRetriever {
    async fn get(&self, url: &str) -> Result<String, KeycloakError> {
        KeycloakAdminToken::acquire(&self.url, &self.username, &self.password, &self.client)
            .await?
            .get(url)
            .await
    }
}

impl KeycloakService {
    pub async fn new(config: &Config) -> BasicResult<Self> {
        let client = Client::new();
        let token = KeycloakAdminTokenRetriever::new(
            config.keycloak_url.clone(),
            config.keycloak_user.clone(),
            config.keycloak_password.clone(),
        );

        let admin = KeycloakAdmin::new(&config.keycloak_url, token, client.clone());

        Ok(Self {
            admin: Arc::new(admin),
            realm: config.keycloak_realm.clone(),
            client,
            url: config.keycloak_url.clone(),
            admin_name: config.admin_user.clone(),
            admin_password: config.admin_password.clone(),
        })
    }

    pub async fn init_realm(&self, user_service: &UserService) -> BasicResult<()> {
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

        if self.check_and_create_role("admin").await? {
            self.create_admin_user(user_service).await?;
        }

        Ok(())
    }

    async fn create_admin_user(&self, user_service: &UserService) -> BasicResult<()> {
        if !self
            .get_users(self.admin_name.clone(), true)
            .await
            .map_err(|e| e.to_string())?
            .is_empty()
        {
            info!("Admin user already exists in keycloak");
            return Ok(());
        }

        info!("Creating admin user in keycloak");
        self.create_user(UserRepresentation {
            username: Some(self.admin_name.clone()),
            email: Some("admin@localhost".to_string()),
            enabled: Some(true),
            email_verified: Some(true),
            credentials: Some(vec![keycloak::types::CredentialRepresentation {
                temporary: Some(false),
                type_: Some("password".to_string()),
                value: Some(self.admin_password.clone()),
                ..Default::default()
            }]),
            realm_roles: Some(vec!["admin".to_string()]),
            ..Default::default()
        })
        .await
        .map_err(|e| e.to_string())?;

        let kc_user = self
            .get_users(self.admin_name.clone(), true)
            .await
            .map_err(|e| e.to_string())?
            .pop()
            .ok_or("Failed to find admin user in keycloak".to_string())?
            .id
            .ok_or("Failed to get keycloak admin user id".to_string())?;

        self.add_roles_to_user(&kc_user, vec!["admin".to_string()])
            .await
            .map_err(|e| e.to_string())?;

        info!("Creating admin user in database");
        user_service
            .insert(user::ActiveModel {
                name: ActiveValue::Set(self.admin_name.clone()),
                external_id: ActiveValue::Set(Some(kc_user)),
                ..Default::default()
            })
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn check_and_create_role(&self, name: &str) -> BasicResult<bool> {
        if self.get_role_by_name(name.to_string()).await.is_err() {
            debug!("Creating role {}", name);
            self.create_role(RoleRepresentation {
                name: Some(name.to_string()),
                ..Default::default()
            })
            .await
            .map_error_to_basic()?;

            Ok(true)
        } else {
            Ok(false)
        }
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
        let mut user = self
            .admin
            .realm_users_with_id_get(self.realm.as_str(), id.as_str())
            .await
            .map_keycloak_error(Some("Keycloak user not found"))?;

        if let Ok(roles) = self
            .get_user_roles(
                user.id
                    .as_ref()
                    .ok_or(HttpResponseError::failed_dependency(Some(
                        "Failed to get user id",
                    )))?,
            )
            .await
        {
            user.realm_roles = Some(
                roles
                    .into_iter()
                    .filter_map(|r| r.name)
                    .filter(|n| !n.starts_with("default-roles-"))
                    .collect(),
            );
        }

        Ok(user)
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

    async fn get_role_by_name(&self, role_name: String) -> WebResult<RoleRepresentation> {
        self.admin
            .realm_roles_with_role_name_get(self.realm.as_str(), &role_name)
            .await
            .map_failed_dependency(Some("Failed to find matching role in keycloak"))
    }

    async fn create_role(&self, role: RoleRepresentation) -> WebResult<()> {
        self.admin
            .realm_roles_post(self.realm.as_str(), role)
            .await
            .map_failed_dependency(Some("Failed to create the role in keycloak"))
    }

    pub async fn add_roles_to_user(&self, user_id: &str, roles: Vec<String>) -> WebResult<()> {
        let roles = join_all(roles.into_iter().map(|role| self.get_role_by_name(role)))
            .await
            .into_iter()
            .collect::<WebResult<Vec<_>>>()?;

        self.admin
            .realm_users_with_id_role_mappings_realm_post(self.realm.as_str(), user_id, roles)
            .await
            .map_failed_dependency(Some("Failed to add role to user in keycloak"))
    }

    pub async fn get_user_roles(&self, user_id: &str) -> WebResult<Vec<RoleRepresentation>> {
        self.admin
            .realm_users_with_id_role_mappings_realm_get(self.realm.as_str(), user_id)
            .await
            .map_failed_dependency(Some("Failed to get user roles in keycloak"))
    }

    pub async fn get_roles(&self) -> WebResult<Vec<String>> {
        self.admin
            .realm_roles_get(self.realm.as_str(), None, None, None, None)
            .await
            .map_failed_dependency(Some("Failed to get roles in keycloak"))?
            .into_iter()
            .map(|r| {
                r.name.ok_or(HttpResponseError::failed_dependency(Some(
                    "Failed to get role name",
                )))
            })
            .collect::<WebResult<Vec<_>>>()
    }

    /*pub async fn update_user(&self, id: &String, user: UserRepresentation) -> WebResult<()> {
        self.admin
            .realm_users_with_id_put(self.realm.as_str(), id.as_str(), user)
            .await
            .map_failed_dependency(Some("Failed to update the user in keycloak"))
    }*/
}
