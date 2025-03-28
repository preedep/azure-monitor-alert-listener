use actix_web::{HttpResponse, Responder, post, web};
use log::{debug, info};
use serde_json::Value;
use crate::domain::models::AzureMonitorAlert;

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
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            HttpResponse::BadRequest().body(format!("❌ Parse error: {}", e))
        }
    }
}
