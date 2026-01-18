use crate::utils::setup_test;

#[tokio::test]
async fn healt_check_works() {
    let (address, _, client) = setup_test().await;

    let url = format!("{}/health_check", &address);
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
