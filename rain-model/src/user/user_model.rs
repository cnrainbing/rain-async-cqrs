use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

pub struct User {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone,FromRow, Deserialize, Serialize)]
pub struct UserAddress {
    pub id: u64,
    pub user_id: u64,
    pub city: String,
    pub post_code: u32,
}

pub struct UserContact {
    pub contact_type: u8,
    pub detail: String,
}