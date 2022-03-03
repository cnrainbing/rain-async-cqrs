use serde::{Deserialize};

#[derive(Deserialize)]
pub struct UserAddressByRegionQuery {
    pub user_id: u64,
}
