use reqwest::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

use crate::helpers::spawn_app;

#[tokio::test]
async fn confirmations_without_toke_are_rejected_with_a_400() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = reqwest::get(&format!("{}/subscriptions/confirm", &app.address))
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn the_link_returned_by_subscribe_returns_a_200_if_called_with_a_valid_token() {
    // Arrange
    let app = spawn_app().await;
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body).await;
    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

    let get_link = |s: &str| {
        let links: Vec<_> = linkify::LinkFinder::new()
            .kinds(&[linkify::LinkKind::Url])
            .links(s)
            .collect();
        assert_eq!(1, links.len());
        links[0].as_str().to_owned()
    };

    let raw_confirmation_link = &get_link(body["HtmlBody"].as_str().unwrap());
    let mut confirmation_link = Url::parse(raw_confirmation_link).unwrap();
    // In the test environment, without the line below,
    // a request can be made without a port being specified.
    // This is a non-issue for production workloads where the DNS domain is enough.
    confirmation_link.set_port(Some(app.port)).unwrap();
    // Let's make sure we don't call random APIs on the web
    assert_eq!("localhost", confirmation_link.host_str().unwrap());

    // Act
    let response = reqwest::get(confirmation_link)
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}
