use std::sync::Arc;
use actix_web::web::ServiceConfig;
use crate::Configs;
use crate::router::user::user_all_router::register_all_user_service;

/// rest register_all_router
pub fn register_all_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    // user router
    register_all_user_service(cfg, configs);
}