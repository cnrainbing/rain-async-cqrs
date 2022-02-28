use async_trait::async_trait;
use sqlx::{MySqlPool};
use sqlx::mysql::MySqlRow;
use anyhow::{Context, Result};
use rain_model::user::{UserAddress};

pub struct UserAddressReadRepository {}

#[async_trait]
pub trait IUserAddressReadRepository {
    /// 读取用户地址
    async fn get_user_address(pool: &MySqlPool, user_id: &u64) -> Result<Vec<UserAddress>>;
}

#[async_trait]
impl IUserAddressReadRepository for UserAddressReadRepository {
    async fn get_user_address(pool: &MySqlPool, user_id: &u64) -> Result<Vec<UserAddress>> {
        let sql: &str = r#"SELECT * FROM t_user_address WHERE id = ?"#;
        let row: Vec<UserAddress> = sqlx::query_as(sql)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .context("根据user_id查询用户地址错误")?;
        Ok(row)
    }
}