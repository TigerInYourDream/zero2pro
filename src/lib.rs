use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, subscribe};
use sea_orm::DatabaseConnection;
use tracing_actix_web::TracingLogger;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub mod configration;
mod entities;
mod routes;

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    // 输出到控制台中
    let formatting_layer = fmt::layer()
        .with_ansi(true)
        .with_thread_ids(false)
        .with_target(true)
        .with_line_number(true);

    // 输出到文件中
    let file_appender = rolling::never("logs", "app.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_ansi(false)
        .with_writer(non_blocking_appender);

    // 注册
    Registry::default()
        .with(env_filter)
        // ErrorLayer 可以让 color-eyre 获取到 span 的信息
        .with(formatting_layer)
        .with(file_layer)
        .init();
}

pub fn run(listener: TcpListener, db: DatabaseConnection) -> Result<Server, std::io::Error> {
    init_tracing();
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            // add middle ware
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
