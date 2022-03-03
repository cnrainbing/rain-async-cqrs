use std::sync::Arc;
use actix_web::web::ServiceConfig;
use crate::Configs;
use crate::router::user::user_address_router::register_user_address_service;

/// rest user_all_router
pub fn register_all_user_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    register_user_address_service(cfg, configs);
}