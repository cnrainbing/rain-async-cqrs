use rain_repository::user::{IUserAddressWriteRepository, UserAddressWriteRepository};
use rain_commands::user::{CreateUserAddressCommand};
use rain_model::user::{UserAddress};
use anyhow::{Result};
use sqlx::MySqlPool;
use rain_core::error::ErrorCode;

pub struct UserAddressAggregate {}

impl UserAddressAggregate {
    pub async fn handle_create_user_address_command(
        pool: &MySqlPool, user_id: &u64, command: &CreateUserAddressCommand,
    ) -> Result<UserAddress, ErrorCode> {
        UserAddressWriteRepository::create(pool, user_id, command).await
    }
}