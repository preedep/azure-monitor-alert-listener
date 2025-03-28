use crate::domain::models::AzureMonitorAlert;
use actix_web::{post, web, HttpResponse, Responder};
use log::{debug, error};
use serde_json::Value;
use crate::application::mail::template_render::render_alert_email;

#[post("/alert")]
pub async fn receive_alert(payload: web::Json<Value>) -> impl Responder {
    debug!(
        "📦 Raw JSON Payload:\n{}",
        serde_json::to_string_pretty(&payload).unwrap()
    );
    match AzureMonitorAlert::try_from(payload) {
        Ok(alert) => {
            //debug!("✅ Got alert: {:?}", alert.data.essentials.alert_rule);
            debug!("Alert payload: {:#?}", alert);
            let ret = render_alert_email("template/*", "mail_template.html", &alert);
            match ret {
                Ok(html) => {
                    debug!("📧 Email HTML:\n{}", html);

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
