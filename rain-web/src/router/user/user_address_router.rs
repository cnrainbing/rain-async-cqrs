use std::sync::Arc;
use actix_web::{HttpResponse, web, get};
use actix_web::web::ServiceConfig;
use rain_core::error::ErrorCode;
use rain_core::response::ServiceResponse;
use crate::{AppState, Configs};
use rain_projections::user::UserAddressProjection as Projection;
use rain_queries::user::{UserAddressByRegionQuery};

#[get("/user/address/region")]
pub async fn user_address_by_region_router(
    query: web::Query<UserAddressByRegionQuery>,
    ctx: web::Data<Arc<AppState>>,
) -> anyhow::Result<HttpResponse, ErrorCode> {
    log::debug!("user_address_by_region -> user_id:{}",query.user_id);

    let result = Projection::handle_user_address_by_region(&ctx.pool, &query).await;
    match result {
        Ok(data) => {
            return Ok(HttpResponse::Ok().json(ServiceResponse::success(data)));
        }
        Err(e) => Err(e)
    }
}

/// rest register_user_address_router
pub fn register_user_address_router(cfg: &mut ServiceConfig, _configs: Arc<Configs>) {
    cfg.service(user_address_by_region_router);
}