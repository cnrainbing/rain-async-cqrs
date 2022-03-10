use async_trait::async_trait;
use sqlx::{Error, MySqlPool};
use rain_base::error::ErrorCode;
use rain_model::user::{UserAddress};
use rain_queries::user::{UserAddressByRegionQuery};

pub struct UserAddressReadRepository {}

#[async_trait]
pub trait IUserAddressReadRepository {
    /// 读取用户地址
    async fn fetch_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> anyhow::Result<Vec<UserAddress>, ErrorCode>;
}

#[async_trait]
impl IUserAddressReadRepository for UserAddressReadRepository {
    async fn fetch_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> anyhow::Result<Vec<UserAddress>, ErrorCode> {
        let sql: &str = r#"SELECT * FROM t_user_address WHERE user_id = ?"#;
        let row: anyhow::Result<Vec<UserAddress>, Error> = sqlx::query_as(sql)
            .bind(&query.user_id)
            .fetch_all(pool)
            .await;
        Err(ErrorCode::DB_ERR)
        /*match row {
            Ok(data) => Ok(data),
            Err(e) => {
                log::debug!("fetch_user_address_by_region -> e:{}",e.as_database_error().unwrap().to_string());
                Err(ErrorCode::DB_ERR)
            }
        }*/
    }
}