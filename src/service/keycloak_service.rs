use crate::config::config::Config;
use crate::error::http_response_error::MapHttpResponseError;
use crate::util::types::{BasicResult, WebResult};
use keycloak::types::{RealmRepresentation, UserRepresentation};
use keycloak::{KeycloakAdmin, KeycloakAdminToken};
use log::debug;
use std::sync::Arc;

#[derive(Clone)]
pub struct KeycloakService {
    admin: Arc<KeycloakAdmin>,
    realm: String,
}

impl KeycloakService {
    pub async fn new(config: &Config) -> BasicResult<Self> {
        let client = reqwest::Client::new();
        let token = KeycloakAdminToken::acquire(
            &config.keycloak_url,
            &config.keycloak_user,
            &config.keycloak_password,
            &client,
        )
        .await?;

        let admin = KeycloakAdmin::new(&config.keycloak_url, token, client);

        Ok(Self {
            admin: Arc::new(admin),
            realm: config.keycloak_realm.clone(),
        })
    }

    pub async fn init_realm(&self) -> BasicResult<()> {
        if self.get_realm(self.realm.as_str()).await.is_ok() {
            debug!("Realm {} already exists", self.realm.as_str());
            return Ok(());
        }

        self.create_realm(RealmRepresentation {
            realm: Some(self.realm.clone()),
            enabled: Some(true),
            ..Default::default()
        })
        .await
    }

    pub async fn get_realm(&self, realm: &str) -> BasicResult<RealmRepresentation> {
        self.admin.realm_get(realm).await.map_err(|e| e.into())
    }

    pub async fn create_realm(&self, realm: RealmRepresentation) -> BasicResult<()> {
        self.admin.post(realm).await.map_err(|e| e.into())
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

    /*pub async fn update_user(&self, id: &String, user: UserRepresentation) -> WebResult<()> {
        self.admin
            .realm_users_with_id_put(self.realm.as_str(), id.as_str(), user)
            .await
            .map_failed_dependency(Some("Failed to update the user in keycloak"))
    }*/
}
