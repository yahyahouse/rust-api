
use sqlx::{Error, PgPool};
use crate::models::User;

pub struct UserRepository {}

impl UserRepository {
    pub async fn create(pool: &PgPool, user: User) -> Result<User, Error> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, name, password, role)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, email, name, password, role",
        )
            .bind(user.id)
            .bind(user.email)
            .bind(user.name)
            .bind(user.password)
            .bind(user.role)
            .fetch_one(pool)
            .await

    }
    pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await
    }
    pub async fn get_user_by_id(pool: &PgPool, id: String) -> Result<User, Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
    }
    pub async fn delete_user_by_id(pool: &PgPool, id: String) -> Result<u64, Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}