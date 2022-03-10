use std::sync::Arc;
use std::io;
use sqlx::{MySqlPool, MySql, Pool};
use anyhow::{Result, Ok};

use actix_web::dev::Server;
use actix_web::middleware::{Logger, Compress};
use actix_web::web::{resource, ServiceConfig, Data};
use actix_web::{App, middleware, guard, HttpServer, web};
//use actix_web_requestid::{RequestIDService};
use guard::{Get};

use crate::config::configs::{Configs, DatabaseConfig};
use crate::router::health_check::health_check;
use crate::router::register_all_router::register_all_service;

/// 工程内部mod
pub mod config;
pub mod router;
pub mod constant;
pub mod enums;

/// 全局的 state
pub struct AppState {
    // 数据库连接池
    pool: Arc<MySqlPool>,
}

impl AppState {
    // 通过 ctx 获取 数据库连接池
    pub fn get_pool(ctx: web::Data<AppState>) -> Result<Arc<Pool<MySql>>> {
        Ok(ctx.pool.clone())
    }
}

/// http server application
pub struct Application {
    server: Server,
}

impl Application {
    /// 构建 服务器
    pub async fn build(configs: Arc<Configs>) -> Result<Application> {
        // 链接数据库
        let pool: Arc<Pool<MySql>> = DatabaseConfig::init(&configs.database).await?;

        let app_state: Arc<AppState> = Arc::new(AppState { pool });

        let address = configs.server.get_address();
        log::info!("🚀http://{}{}",address,"/");

        let server: Server = build_actix_server(configs, address, app_state)?;

        Ok(Application { server })
    }

    /// 启动
    pub async fn run(self) -> Result<(), io::Error> {
        self.server.await
    }
}

/// 构建 服务器
fn build_actix_server(
    configs: Arc<Configs>,
    address: String,
    app_state: Arc<AppState>,
) -> Result<Server> {
    let server: Server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "1.0.0")))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .app_data(Data::new(configs.clone()))
            .app_data(Data::new(app_state.clone()))
            .configure(|cfg| register_service(cfg, configs.clone()))
    })
        .bind(address)?
        .workers(128)
        .max_connections(65535)
        // <- 设置关机超时时间为20s.
        .shutdown_timeout(20)
        .run();
    Ok(server)
}

/// 注册路由 每一个worker都会注册一下
fn register_service(cfg: &mut ServiceConfig, configs: Arc<Configs>) {
    // rest 健康检查
    cfg.service(
        resource(configs.server.get_health_check())
            .guard(Get())
            .to(health_check),
    );

    // rest all
    register_all_service(cfg, configs);
}
