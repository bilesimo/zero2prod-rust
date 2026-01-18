use reqwest::Client;
use sqlx::{Connection, PgConnection};
use zero2prod::{configuration::get_configuration, startup::run};

fn spawn_app() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

async fn get_db_conn() -> PgConnection {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres")
}

pub async fn setup_test() -> (String, PgConnection, Client) {
    let app_adress = spawn_app();
    let connection = get_db_conn().await;
    let client = Client::new();

    (app_adress, connection, client)
}
