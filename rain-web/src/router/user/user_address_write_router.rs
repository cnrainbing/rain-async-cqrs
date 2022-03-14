use std::sync::Arc;
use actix_web::{HttpResponse, web, post};
use actix_web::web::ServiceConfig;
use anyhow::Result;
use rain_core::error::ErrorCode;
use rain_core::response::ServiceResponse;
use crate::{AppState, Configs};
use rain_aggregates::user::UserAddressAggregate as Aggregate;
use rain_commands::user::CreateUserAddressCommand;

#[post("/create/user/address/command")]
pub async fn create_user_address_command_router(
    command: web::Json<CreateUserAddressCommand>,
    ctx: web::Data<Arc<AppState>>,
) -> Result<HttpResponse, ErrorCode> {
    log::debug!("create_user_address_command_router -> city:{}",command.city);
    let user_id = &1;
    let result = Aggregate::handle_create_user_address_command(&ctx.pool, &user_id, &command).await;
    match result {
        Ok(data) => {
            return Ok(HttpResponse::Ok().json(ServiceResponse::success(data)));
        }
        Err(e) => Err(e)
    }
}

/// rest register_user_address_router
pub fn register_user_address_router(cfg: &mut ServiceConfig, _configs: Arc<Configs>) {
    cfg.service(create_user_address_command_router);
}