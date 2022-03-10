use rain_repository::user::{IUserAddressReadRepository, UserAddressReadRepository as ReadRepository};
use rain_queries::user::{UserAddressByRegionQuery};
use rain_model::user::{UserAddress};
use anyhow::{Result};
use sqlx::MySqlPool;
use rain_core::error::ErrorCode;

pub struct UserAddressProjection {}

impl UserAddressProjection {
    pub async fn handle_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> Result<Vec<UserAddress>, ErrorCode> {
        ReadRepository::fetch_user_address_by_region(pool, query).await
    }
}