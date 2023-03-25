use crate::entity::token;
use crate::util::types::DbResult;
use futures_util::future::join_all;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter,
};
use uuid::Uuid;

pub struct TokenRepository;

impl TokenRepository {
    pub async fn find_by_id<C>(
        db: &C,
        id: &Uuid,
        include_inactive: bool,
    ) -> DbResult<Option<token::Model>>
    where
        C: ConnectionTrait,
    {
        let mut q = token::Entity::find_by_id(id.clone());
        if !include_inactive {
            q = q.filter(token::Column::Active.eq(true));
        }

        q.one(db).await
    }

    pub async fn find_all_by_client<C>(
        db: &C,
        client_id: &Uuid,
        include_inactive: bool,
    ) -> DbResult<Vec<token::Model>>
    where
        C: ConnectionTrait,
    {
        let mut q = token::Entity::find().filter(token::Column::ClientId.eq(client_id.clone()));

        if !include_inactive {
            q = q.filter(token::Column::Active.eq(true));
        }

        q.all(db).await
    }

    pub async fn deactivate_all_by_client<C>(
        db: &C,
        client_id: &Uuid,
    ) -> DbResult<Vec<token::Model>>
    where
        C: ConnectionTrait,
    {
        let mut tokens = Self::find_all_by_client(db, client_id, false).await?;
        for token in tokens.iter_mut() {
            let mut token = token.clone().into_active_model();
            token.active = ActiveValue::Set(false);
            token.into_active_model().update(db).await?;
        }

        Ok(tokens)
    }

    pub async fn delete_all_by_client<C>(db: &C, client_id: &Uuid) -> DbResult<Vec<DeleteResult>>
    where
        C: ConnectionTrait,
    {
        join_all(
            Self::find_all_by_client(db, client_id, true)
                .await?
                .into_iter()
                .map(|t| t.delete(db)),
        )
        .await
        .into_iter()
        .collect::<DbResult<Vec<_>>>()
    }
}
