use crate::domain::models::AzureMonitorAlert;
use actix_web::{post, web, HttpResponse, Responder};
use log::{debug, error};
use serde_json::Value;
use crate::application::mail::mail_sender::send_email_with_api;
use crate::application::mail::template_render::render_alert_email;
use crate::interface::state::AppState;

#[post("/alert")]
pub async fn receive_alert(app_state: web::Data<AppState>,payload: web::Json<Value>) -> impl Responder {
    debug!(
        "📦 Raw JSON Payload:\n{}",
        serde_json::to_string_pretty(&payload).unwrap()
    );
    match AzureMonitorAlert::try_from(payload) {
        Ok(alert) => {
            let state = app_state.get_ref();
            //debug!("✅ Got alert: {:?}", alert.data.essentials.alert_rule);
            debug!("Alert payload: {:#?}", alert);
            let ret = render_alert_email("template/*", "mail_template.html", &alert);
            match ret {
                Ok(html) => {
                    debug!("📧 Email HTML:\n{}", html);
                    let res = send_email_with_api(
                        &state.tenant_id,
                        &state.client_id,
                        &state.client_secret,
                        &state.asc_url,
                        &state.sender,
                        &state.reply_to,
                        &state.display_name,
                        &alert.data.essentials.alert_rule,
                        &html,
                    ).await;
                    match res {
                        Ok(_) => {
                            debug!("✉️ Email sent successfully");
                        }
                        Err(e) => {
                            error!("❌ Error sending email: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("❌ Error rendering email: {}", e);
                }
            }
            HttpResponse::Ok().finish()
        }
        Err(e) => HttpResponse::BadRequest().body(format!("❌ Parse error: {}", e)),
    }
}
