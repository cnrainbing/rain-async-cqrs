use async_trait::async_trait;
use sqlx::{MySqlPool};
use anyhow::{Context, Result};
use rain_model::user::{UserAddress};
use rain_commands::user::{CreateUserAddressCommand};

pub struct UserAddressWriteRepository {}

#[async_trait]
pub trait IUserAddressWriteRepository {
    /// 添加用户地址
    async fn create(pool: &MySqlPool, user_id: &u64, new_user_address: &CreateUserAddressCommand) -> Result<UserAddress>;
}

#[async_trait]
impl IUserAddressWriteRepository for UserAddressWriteRepository {
    async fn create(pool: &MySqlPool, user_id: &u64, command: &CreateUserAddressCommand) -> Result<UserAddress> {
        let sql: &str = r#"INSERT INTO t_user_address(user_id,city,post_code) VALUES (?,?,?)"#;
        let insert_id: u64 = sqlx::query(sql)
            .bind(&user_id)
            .bind(&command.city)
            .bind(&command.post_code)
            .execute(pool)
            .await.context("创建用户地址失败")?
            .last_insert_id();
        let new_user_address: UserAddress = UserAddress {
            id: insert_id,
            user_id: *user_id,
            city: command.city.to_string(),
            post_code: command.post_code,
        };
        Ok(new_user_address)
    }
}