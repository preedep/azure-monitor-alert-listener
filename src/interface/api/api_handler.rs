use crate::application::log::log_analytic_query::query_log_link;
use crate::application::mail::mail_sender::send_email_with_api;
use crate::application::mail::template_render::render_alert_email;
use crate::domain::models::{AlertContext, AzureMonitorAlert, LogAnalyticsResponse};
use crate::interface::state::AppState;
use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use log::{debug, error, info};
use serde_json::Value;

/// Actix handler for the `/alert` route.
#[post("/alert")]
pub async fn receive_alert(
    app_state: web::Data<AppState>,
    payload: web::Json<Value>,
) -> HttpResponse {
    handle_alert(app_state, payload).await
}

/// Actix handler for the `/alert_secure` route.
#[post("/alert_secure")]
pub async fn receive_alert_secure(
    req: HttpRequest,
    app_state: web::Data<AppState>,
    payload: web::Json<Value>,
) -> HttpResponse {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().body("Missing Authorization header"),
    };

    if !auth_header.starts_with("Bearer ") {
        return HttpResponse::Unauthorized().body("Invalid token format");
    }

    let token = &auth_header[7..];
    debug!("Token received: {}", token);

    match app_state
        .jwt_verifier
        .verify(token, &app_state.tenant_id, "api://your-app-id-uri")
        .await
    {
        Ok(claims) => {
            info!("✅ Verified secure alert from aud: {}", claims.aud);
            handle_alert(app_state, payload).await
        }
        Err(e) => {
            error!("❌ Invalid JWT: {}", e);
            HttpResponse::Unauthorized().body(format!("Invalid JWT: {}", e))
        }
    }
}

async fn handle_alert(app_state: web::Data<AppState>, payload: web::Json<Value>) -> HttpResponse {
    debug!(
        "📦 Raw JSON Payload:\n{}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let mut alert = match AzureMonitorAlert::try_from(payload) {
        Ok(alert) => alert,
        Err(e) => return HttpResponse::BadRequest().body(format!("❌ Parse error: {}", e)),
    };

    let app_state_ref = app_state.get_ref();

    if let Some(alert_context) = &alert.data.alert_context {
        let r = process_alert(app_state_ref, alert_context).await;
        alert.data.pipeline_name = Some(r.0);
        alert.data.message = Some(r.1);
    } else {
        debug!("No alert context present in payload.");
    }

    match render_and_send_email(app_state_ref, &alert).await {
        Ok(_) => debug!("✉️ Email processed and sent successfully."),
        Err(e) => error!("❌ Error in email processing: {}", e),
    }

    HttpResponse::Ok().finish()
}

async fn process_alert(app_state_ref: &AppState, alert_context: &AlertContext) -> (String, String) {
    if let Some(first_condition) = alert_context
        .condition
        .all_of
        .as_ref()
        .and_then(|all_of| all_of.get(0))
    {
        debug!("First condition: {:#?}", first_condition);

        if let Some(search_query) = &first_condition.search_query {
            debug!("Search query found: {}", search_query);

            let timespan = format!(
                "{}/{}",
                alert_context.condition.window_start_time, alert_context.condition.window_end_time
            );

            match query_log_link(
                &app_state_ref.tenant_id,
                &app_state_ref.client_id,
                &app_state_ref.client_secret,
                &app_state_ref.workspace_id,
                search_query,
                &timespan,
            )
            .await
            {
                Ok(log_response) => {
                    return process_log_condition(log_response);
                }
                Err(e) => {
                    error!("❌ Error querying Log Analytics: {}", e)
                }
            }
        }
    }
    (String::new(), String::new())
}

fn process_log_condition(log_response: LogAnalyticsResponse) -> (String, String) {
    let mut pipeline_names = Vec::new();
    let mut error_messages = Vec::new();

    for table in log_response.tables {
        for row in &table.rows {
            if let Some(pipeline_name) = table
                .get_by_column_name(row, "PipelineName")
                .and_then(|v| v.as_str())
            {
                pipeline_names.push(pipeline_name.to_string());
            }
            if let Some(error_message) = table
                .get_by_column_name(row, "ConcatenatedErrors")
                .and_then(|v| v.as_str())
            {
                error_messages.push(error_message.to_string());
            }
        }
    }
    let pipeline_names = pipeline_names.join(", ");
    let error_messages = error_messages.join(", ");

    (pipeline_names, error_messages)
}

async fn render_and_send_email(
    app_state_ref: &AppState,
    alert: &AzureMonitorAlert,
) -> Result<(), String> {
    match render_alert_email("template/*", "mail_template.html", alert) {
        Ok(rendered_email) => {
            debug!("📧 Generated email HTML:\n{}", rendered_email);

            send_email_with_api(
                &app_state_ref.tenant_id,
                &app_state_ref.client_id,
                &app_state_ref.client_secret,
                &app_state_ref.asc_url,
                &app_state_ref.sender,
                &app_state_ref.reply_to,
                &app_state_ref.display_name,
                &alert.data.essentials.alert_rule,
                &rendered_email,
            )
            .await
            .map_err(|e| {
                error!("❌ Error sending email: {}", e);
                format!("Error sending email: {}", e)
            })
        }
        Err(e) => {
            error!("❌ Error rendering email: {}", e);
            Err(format!("Error rendering email: {}", e))
        }
    }
}
