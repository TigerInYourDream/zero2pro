use sea_orm::Database;
use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let setting =
        zero2prod::configration::get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", setting.application_port))?;
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        setting.database.username,
        setting.database.password,
        setting.database.host,
        setting.database.port,
        setting.database.database_name
    );

    let db = Database::connect(database_url).await.unwrap();
    run(listener, db)?.await
}
