use crate::constants::mongo_error_codes::mongodb_error_codes;
use crate::errors::repositories_error::RepositoryError;
use crate::models::auth::AuthTokenInDB;
use crate::models::user::UserInDB;
use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime as BsonDateTime};
use mongodb::{error::Error as MongoError, Collection, Database};

#[async_trait]
pub trait AuthRepository {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserInDB>, RepositoryError>;
    async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        username: &str,
    ) -> Result<ObjectId, RepositoryError>;
    async fn save_auth_token(&self, auth_token: &AuthTokenInDB) -> Result<(), RepositoryError>;
    async fn delete_auth_tokens(&self, access_token: &str) -> Result<bool, RepositoryError>;
    async fn find_auth_token(&self, token: &str) -> Result<Option<AuthTokenInDB>, RepositoryError>;
    async fn find_auth_token_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<Option<AuthTokenInDB>, RepositoryError>;
}

pub struct MongoAuthRepository {
    users_collection: Collection<UserInDB>,
    tokens_collection: Collection<AuthTokenInDB>,
}

impl MongoAuthRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            users_collection: db.collection("users"),
            tokens_collection: db.collection("auth_tokens"),
        }
    }
}

#[async_trait]
impl AuthRepository for MongoAuthRepository {
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserInDB>, RepositoryError> {
        self.users_collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn create_user(
        &self,
        email: &str,
        password_hash: &str,
        username: &str,
    ) -> Result<ObjectId, RepositoryError> {
        let user_in_db = UserInDB {
            id: None,
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            username: username.to_string(),
            created_at: BsonDateTime::now(),
            updated_at: None,
        };

        match self.users_collection.insert_one(&user_in_db, None).await {
            Ok(result) => result
                .inserted_id
                .as_object_id()
                .ok_or(RepositoryError::DatabaseError(MongoError::custom(
                    "挿入されたドキュメントのIDが無効です",
                ))),
            Err(e) => {
                if let mongodb::error::ErrorKind::Write(write_failure) = e.kind.as_ref() {
                    if let mongodb::error::WriteFailure::WriteError(write_error) = write_failure {
                        if write_error.code == mongodb_error_codes::DUPLICATE_KEY {
                            return Err(RepositoryError::DuplicateError(
                                "メールアドレスが既に使用されています".to_string(),
                            ));
                        }
                    }
                }
                Err(RepositoryError::DatabaseError(e))
            }
        }
    }

    async fn save_auth_token(&self, auth_token: &AuthTokenInDB) -> Result<(), RepositoryError> {
        self.tokens_collection
            .insert_one(auth_token, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(())
    }

    async fn delete_auth_tokens(&self, access_token: &str) -> Result<bool, RepositoryError> {
        let result = self
            .tokens_collection
            .delete_one(doc! { "access_token": access_token }, None)
            .await
            .map_err(RepositoryError::DatabaseError)?;

        Ok(result.deleted_count > 0)
    }

    async fn find_auth_token(&self, token: &str) -> Result<Option<AuthTokenInDB>, RepositoryError> {
        self.tokens_collection
            .find_one(doc! { "access_token": token }, None)
            .await
            .map_err(RepositoryError::DatabaseError)
    }

    async fn find_auth_token_by_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<Option<AuthTokenInDB>, RepositoryError> {
        self.tokens_collection
            .find_one(doc! { "refresh_token": refresh_token }, None)
            .await
            .map_err(RepositoryError::DatabaseError)
    }
}
