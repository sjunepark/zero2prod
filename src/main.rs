use zero2prod::configuration::get_configuration;
use zero2prod::startup::Application;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    Application::build(configuration)
        .await?
        .run_until_stopped()
        .await?;

    Ok(())
}
