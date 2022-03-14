use async_trait::async_trait;
use sqlx::{MySqlPool};
use anyhow::{Result};
use rain_model::user::{UserAddress};
use rain_commands::user::{CreateUserAddressCommand};
use rain_core::error::ErrorCode;

pub struct UserAddressWriteRepository {}

#[async_trait]
pub trait IUserAddressWriteRepository {
    /// 添加用户地址
    async fn create(pool: &MySqlPool, user_id: &u64, new_user_address: &CreateUserAddressCommand) -> Result<UserAddress, ErrorCode>;
}

#[async_trait]
impl IUserAddressWriteRepository for UserAddressWriteRepository {
    async fn create(pool: &MySqlPool, user_id: &u64, command: &CreateUserAddressCommand) -> Result<UserAddress, ErrorCode> {
        let sql: &str = r#"INSERT INTO t_user_address(user_id,city,post_code) VALUES (?,?,?)"#;
        let row = sqlx::query(sql)
            .bind(&user_id)
            .bind(&command.city)
            .bind(&command.post_code)
            .execute(pool).await;

        match row {
            Ok(query_result) => {
                let new_user_address = UserAddress {
                    id: query_result.last_insert_id(),
                    user_id: *user_id,
                    city: command.city.to_string(),
                    post_code: command.post_code,
                };
                Ok(new_user_address)
            }
            Err(_e) => { Err(ErrorCode::CREATE_USER_ADDRESS_ERR) }
        }
    }
}