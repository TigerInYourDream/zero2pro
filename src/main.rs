use sea_orm::Database;
use secrecy::ExposeSecret;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let setting = zero2prod::configration::get_configuration().unwrap();
    let listener = TcpListener::bind(format!(
        "{}:{}",
        setting.application.host, setting.application.port
    ))?;
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        setting.database.username,
        setting.database.password.expose_secret(),
        setting.database.host,
        setting.database.port,
        setting.database.db_name
    );

    let db = Database::connect(database_url).await.unwrap();
    run(listener, db)?.await
}
