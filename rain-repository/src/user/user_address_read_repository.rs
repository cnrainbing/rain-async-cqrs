use async_trait::async_trait;
use sqlx::{MySqlPool};
use anyhow::{Context, Result};
use rain_model::user::{UserAddress};
use rain_queries::user::{UserAddressByRegionQuery};

pub struct UserAddressReadRepository {}

#[async_trait]
pub trait IUserAddressReadRepository {
    /// 读取用户地址
    async fn get_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> Result<Vec<UserAddress>>;
}

#[async_trait]
impl IUserAddressReadRepository for UserAddressReadRepository {
    async fn get_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> Result<Vec<UserAddress>> {
        let sql: &str = r#"SELECT * FROM t_user_address WHERE user_id = ?"#;
        let row: Vec<UserAddress> = sqlx::query_as(sql)
            .bind(&query.user_id)
            .fetch_all(pool)
            .await
            .context("根据user_id查询用户地址错误")?;
        Ok(row)
    }
}