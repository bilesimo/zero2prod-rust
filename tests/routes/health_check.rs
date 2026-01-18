use crate::utils::spawn_app;

#[tokio::test]
async fn healt_check_works() {
    let app = spawn_app().await;

    let url = format!("{}/health_check", &app.address);
    let response = app
        .client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
