use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres, query, query_as};
use uuid::Uuid;

use crate::model::user::{NewUser, User};

#[async_trait]
pub trait UserRepository: UserRepositoryClone {
    async fn create(&self, p_user: &NewUser) -> Result<User, Error>;
    async fn update(&self, p_user: &User) -> Result<User, Error>;
    async fn find_by_user_id(&self, p_user_id: String) -> Result<Option<User>, Error>;
    async fn find_by_identifier(&self, p_identifier: Uuid) -> Result<Option<User>, Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn delete_by_identifier(&self, p_identifier: Uuid) -> Result<(), Error>;
}

pub trait UserRepositoryClone: Send + Sync {
    fn clone_box(&self) -> Box<dyn UserRepository>;
}

impl<U> UserRepositoryClone for U
    where
        U: 'static + UserRepository + Clone,
{
    fn clone_box(&self) -> Box<dyn UserRepository> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserRepository> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: Arc<Pool<Postgres>>,
}

impl UserRepositoryImpl {
    pub fn new(pool: Arc<Pool<Postgres>>) -> UserRepositoryImpl {
        UserRepositoryImpl { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, p_user: &NewUser) -> Result<User, Error> {
        let mut tx = self.pool.begin().await?;

        let user = query_as::<_, User>(
            r#"
                insert into user_entity (
                    version,
                    identifier,
                    first_name,
                    last_name,
                    user_id,
                    admin)
                values (1, $1,$2,$3,$4, false)
                returning *"#,
        )
            .bind(&p_user.identifier)
            .bind(&p_user.first_name)
            .bind(&p_user.last_name)
            .bind(&p_user.user_id)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(user)
    }

    async fn update(&self, p_user: &User) -> Result<User, Error> {
        let mut tx = self.pool.begin().await?;

        let user = query_as::<_, User>(
            r#"
                update user_entity set
                    version = $1,
                    first_name = $2,
                    last_name = $3,
                    admin = $4
                where identifier = $5 and version = $6
                returning *"#,
        )
            .bind(&p_user.version + 1)
            .bind(&p_user.first_name)
            .bind(&p_user.last_name)
            .bind(&p_user.admin)
            .bind(&p_user.identifier)
            .bind(&p_user.version)
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(user)
    }

    async fn find_by_user_id(&self, p_user_id: String) -> Result<Option<User>, Error> {
        let user = query_as::<_, User>("select * from user_entity where user_id = $1")
            .bind(p_user_id)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user)
    }

    async fn find_by_identifier(&self, p_identifier: Uuid) -> Result<Option<User>, Error> {
        let user = query_as::<_, User>("select * from user_entity where identifier = $1")
            .bind(p_identifier)
            .fetch_optional(&*self.pool)
            .await?;

        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>, Error> {
        let users =
            query_as::<_, User>("select * from user_entity order by first_name, last_name, id")
                .fetch_all(&*self.pool)
                .await?;

        Ok(users)
    }

    async fn delete_by_identifier(&self, p_identifier: Uuid) -> Result<(), Error> {
        let mut tx = self.pool.begin().await?;

        query("delete from user_entity where identifier = $1")
            .bind(p_identifier)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}
