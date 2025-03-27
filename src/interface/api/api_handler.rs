use actix_web::{post, web, HttpResponse, Responder};
use log::{debug, info};
use serde_json::Value;

#[post("/alert")]
pub async fn receive_alert(payload: web::Json<Value>) -> impl Responder {

    debug!("📦 Raw JSON Payload:\n{}", serde_json::to_string_pretty(&payload).unwrap());

    // ลอง extract field แบบ safe (optional)
    if let Some(schema_id) = payload.get("schemaId") {
        info!("🔍 schemaId: {}", schema_id);
    }

    if let Some(data) = payload.get("data") {
        if let Some(essentials) = data.get("essentials") {
            if let Some(alert_rule) = essentials.get("alertRule") {
                info!("🚨 Alert Rule: {}", alert_rule);
            }
            if let Some(severity) = essentials.get("severity") {
                info!("⚠️ Severity: {}", severity);
            }
        }
    }

    HttpResponse::Ok().finish()
}