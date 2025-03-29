use crate::domain::models::LogAnalyticsResponse;
use azure_core::HttpClient;
use azure_core::auth::TokenCredential;
use azure_identity::ClientSecretCredential;
use log::{debug, error};
use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use url::Url;

// Adding a function to create a `HttpClient`
fn create_http_client() -> Arc<dyn HttpClient> {
    // Use `azure_core::new_http_client` to create a compatible HTTP client
    azure_core::new_http_client()
}

pub async fn query_log_link(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    workspace_id: &str,
    kql_query: &str,
    timespan: &str,
) -> Result<LogAnalyticsResponse, Box<dyn std::error::Error>> {
    let http_client = create_http_client();

    let token_url = "https://login.microsoftonline.com/";

    let credential = ClientSecretCredential::new(
        http_client,
        Url::parse(&token_url).unwrap(),
        tenant_id.to_string(),
        client_id.to_string(),
        client_secret.to_string(),
    );

    // Get the token and handle errors using `?`
    let token = credential
        .get_token(&["https://api.loganalytics.io/.default"])
        .await?; // This now contains the `AccessToken`

    // Use the token directly without a match block
    debug!("Token: {:?}", token.token.secret());
    debug!("Token Expires: {:?}", token.expires_on);
    debug!("kql_query: {}", kql_query);
    debug!("timespan: {}", timespan);

    let url = format!(
        "https://api.loganalytics.io/v1/workspaces/{}/query",
        workspace_id
    );
    debug!("URL: {}", url);

    let body = json!({
        "query": kql_query,
        "timespan": timespan,
    });
    debug!("Body: {}", body);

    let client = Client::new();
    let response = client
        .post(&url)
        .bearer_auth(token.token.secret())
        .json(&body)
        .send()
        .await?;
    let response = response.json::<LogAnalyticsResponse>().await?;
    Ok(response)
}
