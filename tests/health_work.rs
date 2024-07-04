use std::net::TcpListener;

use sea_orm::Database;

#[tokio::test]
async fn health_check_work() {
    spwan_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    eprintln!("{}", response.status());
    assert_eq!(Some(0), response.content_length())
}

async fn spwan_app() {
    let setting =
        zero2prod::configration::get_configuration().expect("Failed to read configuration.");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", setting.application.port)).unwrap();
    let database_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        setting.database.username,
        setting.database.password,
        setting.database.host,
        setting.database.port,
        setting.database.db_name
    );

    let db = Database::connect(database_url).await.unwrap();
    let server = zero2prod::run(listener, db).expect("Failed to bind address.");
    let _ = tokio::spawn(server);
}
