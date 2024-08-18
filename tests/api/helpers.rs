use std::sync::LazyLock;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

/// Spin up an instance of our application
/// and return its address (i.e. http://localhost:XXXX)
pub async fn spawn_app() -> TestApp {
    *TRACING;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().to_string();
        c.application.port = 0;
        c
    };

    let db_pool = configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");

    let address = format!("http://127.0.0.1:{}", application.port());
    tokio::spawn(application.run_until_stopped());

    TestApp { address, db_pool }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create the database
    let connection = PgPoolOptions::new().connect_lazy_with(config.without_db());
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate the database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database.");

    connection_pool
}
