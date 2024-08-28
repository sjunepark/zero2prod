use std::sync::LazyLock;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgPool};
use uuid::Uuid;
use wiremock::MockServer;

use zero2prod::configuration::{get_configuration, DatabaseSettings};
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let subscriber = get_subscriber("info".into());
    init_subscriber(subscriber);
});

/// Spin up an instance of our application
/// and return its address (i.e. http://localhost:XXXX)
pub async fn spawn_app() -> TestApp {
    *TRACING;

    let email_server = MockServer::start().await;

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        // Use the mock server as email API
        c.email_client.base_url = email_server.uri();
        c
    };

    let db_pool = configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    tokio::spawn(application.run_until_stopped());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool,
        email_server,
        test_user: TestUser::generate(),
    };
    test_app.test_user.store(&test_app.db_pool).await;
    test_app
}

pub struct TestApp {
    pub address: String,
    /// This is necessary because in test environments,
    /// an application url can be used to send requests before a test server is started.
    /// This is a non-issue for production workloads where the DNS domain is enough.
    pub port: u16,
    pub db_pool: PgPool,
    pub email_server: MockServer,
    test_user: TestUser,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: &str) -> reqwest::Response {
        let client = reqwest::Client::new();
        client
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body.to_string())
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_newsletters(&self, body: serde_json::Value) -> reqwest::Response {
        reqwest::Client::new()
            .post(&format!("{}/newsletters", &self.address))
            .basic_auth(&self.test_user.username, Some(&self.test_user.password))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .kinds(&[linkify::LinkKind::Url])
                .links(s)
                .collect();
            assert_eq!(1, links.len());
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();

            // Let's make sure we don't call random APIs on the web
            assert_eq!("localhost", confirmation_link.host_str().unwrap());
            // In the test environment, without the line below,
            // a request can be made without a port being specified.
            // This is a non-issue for production workloads where the DNS domain is enough.
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(&body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(&body["TextBody"].as_str().unwrap());
        ConfirmationLinks { html, plain_text }
    }
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

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

pub struct TestUser {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            user_id: Uuid::new_v4(),
            username: Uuid::new_v4().to_string(),
            password: Uuid::new_v4().to_string(),
        }
    }

    async fn store(&self, pool: &PgPool) {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = Argon2::default()
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        sqlx::query!(
            r#"INSERT into users (user_id, username, password_hash) VALUES ($1, $2, $3)"#,
            self.user_id,
            self.username,
            password_hash
        )
        .execute(pool)
        .await
        .expect("Failed to insert test user.");
    }
}
