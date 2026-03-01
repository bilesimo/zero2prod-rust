use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Debug: check if DO is injecting env vars at all
    tracing::info!(
        raw_host = std::env::var("APP_DATABASE__HOST").unwrap_or("NOT SET".into()),
        raw_port = std::env::var("APP_DATABASE__PORT").unwrap_or("NOT SET".into()),
        raw_user = std::env::var("APP_DATABASE__USERNAME").unwrap_or("NOT SET".into()),
        raw_db = std::env::var("APP_DATABASE__DATABASE_NAME").unwrap_or("NOT SET".into()),
        "Raw environment variables check",
    );

    let configuration = get_configuration().expect("Failed to read configuration.");

    tracing::info!(
        host = configuration.database.host,
        port = configuration.database.port,
        database_name = configuration.database.database_name,
        username = configuration.database.username,
        require_ssl = configuration.database.require_ssl,
        "Config after parsing",
    );

    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    let address = configuration.application.address_string();
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
