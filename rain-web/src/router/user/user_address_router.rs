use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web, get};
use actix_web::web::ServiceConfig;
use crate::{AppState, Configs};
use crate::response::ServiceResponse;
use rain_projections::user::UserAddressProjection as Projection;
use rain_queries::user::{UserAddressByRegionQuery};

#[get("/address/region")]
pub async fn user_address_by_region_router(
    query: web::Query<UserAddressByRegionQuery>,
    ctx: web::Data<Arc<AppState>>,
) -> impl Responder {
    log::debug!("user_address_by_region -> user_id:{}",query.user_id);

    let data = Projection::handle_user_address_by_region(&ctx.pool, &query).await;
    if data.is_ok() {
        HttpResponse::Ok().json(ServiceResponse::success(data.unwrap()))
    } else {
        HttpResponse::Ok().json(ServiceResponse::<Vec<u8>>::default_t(Vec::new()))
    }
}

/// rest register_user_address_router
pub fn register_user_address_router(cfg: &mut ServiceConfig, _configs: Arc<Configs>) {
    cfg.service(user_address_by_region_router);
}