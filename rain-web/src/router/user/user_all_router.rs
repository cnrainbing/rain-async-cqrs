use std::sync::Arc;
use actix_web::web::ServiceConfig;
use crate::Configs;
use crate::router::user::user_address_read_router;
use crate::router::user::user_address_write_router;

/// rest user_all_router
pub fn register_all_user_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    user_address_read_router::register_user_address_router(cfg, configs.clone());
    user_address_write_router::register_user_address_router(cfg, configs.clone());
}