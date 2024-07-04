use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use routes::{health_check, publish_newsletter, subscribe};
use sea_orm::DatabaseConnection;
use time::macros::offset;
use tracing_actix_web::TracingLogger;
use tracing_rolling::{Checker, Daily};

pub mod configration;
mod entities;
mod routes;

pub fn init_tracing() {
    let (writer, token) = Daily::new("logs/app.log", "[year][month][day]", offset!(+8))
        // Daily::new("logs/testing.log", None, offset!(+8))
        .buffered() // buffer file if needed
        .build()
        .unwrap();

    tracing_subscriber::fmt()
        .with_ansi(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .with_writer(writer)
        .init();

    drop(token);
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
            .route("/publish_newsletter", web::post().to(publish_newsletter))
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
