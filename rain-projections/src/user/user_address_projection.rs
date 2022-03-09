use rain_repository::user::{IUserAddressReadRepository, UserAddressReadRepository};
use rain_queries::user::{UserAddressByRegionQuery};
use rain_model::user::{UserAddress};
use anyhow::{Result};
use sqlx::MySqlPool;

pub struct UserAddressProjection {}

impl UserAddressProjection {
    pub async fn handle_user_address_by_region(pool: &MySqlPool, query: &UserAddressByRegionQuery) -> Result<Vec<UserAddress>> {
        UserAddressReadRepository::fetch_user_address_by_region(pool, query).await
    }
}