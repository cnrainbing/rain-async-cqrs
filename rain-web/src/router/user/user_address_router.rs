use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web, get};
use actix_web::web::ServiceConfig;
use crate::{AppState, Configs};
use rain_projections::user::UserAddressProjection as UAP;
use rain_queries::user::{UserAddressByRegionQuery};

#[get("/address/region")]
async fn user_address_by_region(
    query: web::Query<UserAddressByRegionQuery>,
    ctx: web::Data<Arc<AppState>>,
) -> impl Responder {
    log::debug!("user_address_by_region -> user_id:{}",query.user_id);

    let data = UAP::handle_user_address_by_region(&ctx.pool, &query).await;
    HttpResponse::Ok().json(data.unwrap())
}

/// rest register_user_address_service
pub fn register_user_address_service(cfg: &mut ServiceConfig, _configs: Arc<Configs>) {
    cfg.service(user_address_by_region);
}