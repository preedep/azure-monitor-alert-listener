use azure_ecs_rs::adapters::gateways::acs_email::ACSClientBuilder;
use azure_ecs_rs::domain::entities::models::{EmailAddress, EmailContent, Recipients, SentEmailBuilder};
use log::{debug,  info};


/// Sends an email using the ACS client.
///
/// # Arguments
///
/// * `auth_method` - The authentication method to use.
/// * `sender` - The sender's email address.
/// * `recipient` - The recipient's email address.
/// * `display_name` - The display name for the recipient.
pub async fn send_email_with_api(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    asc_url: &str,
    sender: &str,
    recipient: &str,
    display_name: &str,
    subject: &str,
    html_content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Using Service Principal");
    let host_name = asc_url.to_string();

    debug!("host_name: {}", host_name);
    debug!("tenant_id: {}", tenant_id);
    debug!("client_id: {}", client_id);
    debug!("client_secret: {}", client_secret);
    let builder = ACSClientBuilder::new()
        .host(host_name.as_str())
        .service_principal(
            tenant_id,
            client_id,
            client_secret,
        );

    let email_request = SentEmailBuilder::new()
        .sender(sender.to_owned())
        .content(EmailContent {
            subject: Some(subject.to_string()),
            plain_text: None,
            html: Some(html_content.to_string()),
        })
        .recipients(Recipients {
            to: Some(vec![EmailAddress {
                email: Some(recipient.to_owned()),
                display_name: Some(display_name.to_owned()),
            }]),
            cc: None,
            b_cc: None,
        })
        .user_engagement_tracking_disabled(false)
        .build()?;

    debug!("Email request: {:#?}", email_request);

    let acs_client = builder
        .build()?;

    let resp_send_email = acs_client.send_email(&email_request).await.expect("Failed to send email");
    debug!("Response: {:#?}", resp_send_email);
    Ok(())
}