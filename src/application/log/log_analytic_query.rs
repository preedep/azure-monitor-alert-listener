use std::sync::Arc;
use azure_core::{HttpClient};
use azure_identity::ClientSecretCredential;
use azure_core::auth::TokenCredential;
use log::{debug, error};
use url::Url;

// Adding a function to create a `HttpClient`
fn create_http_client() -> Arc<dyn HttpClient> {
    // Use `azure_core::new_http_client` to create a compatible HTTP client
    azure_core::new_http_client()
}
pub async fn query_log_link(tenant_id: &str,
                            client_id :&str,
                            client_secret:&str,
                            log_link:&str) {

    let http_client = create_http_client();

    let token_url = "https://login.microsoftonline.com/";

    let credential = ClientSecretCredential::new(
        http_client,
        Url::parse(&token_url).unwrap(),
        tenant_id.to_string(),
        client_id.to_string(),
        client_secret.to_string(),
    );
    let token = credential
        .get_token(&["https://api.loganalytics.io/.default"])
        .await
        .map_err(|e| format!("Failed to get access token: {}", e));

    match token {
        Ok(token) => {
            let log_link = Url::parse(log_link).unwrap();
            let response = reqwest::Client::new()
                .get(log_link)
                .bearer_auth(token.token.secret())
                .send()
                .await
                .map_err(|e| format!("Failed to send request: {}", e));
            match response {
                Ok(resp) => {
                    debug!("Response: {:?}", resp);
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}