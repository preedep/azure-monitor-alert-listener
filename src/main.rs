mod domain;
mod interface;
mod application;
mod infrastructure;

use actix_web::HttpServer;
use log::info;
use crate::infrastructure::auth::jwt::JwtVerifier;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    info!("Starting Azure Monitor Alert Listener on port {}", port);

    let tenant_id = std::env::var("TENANT_ID").expect("TENANT_ID must be set");
    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set");
    let client_secret = std::env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");
    let asc_url = std::env::var("ASC_URL").expect("ASC_URL must be set");
    let sender = std::env::var("SENDER").expect("SENDER must be set");
    let reply_to = std::env::var("REPLY_EMAIL").expect("REPLY_TO must be set");
    let display_name = std::env::var("REPLY_EMAIL_DISPLAY").expect("DISPLAY_NAME must be set");
    let workspace_id = std::env::var("WORKSPACE_ID").expect("WORKSPACE_ID must be set");


    // 🔐 สร้าง verifier instance
    let jwt_verifier = JwtVerifier::new();

    let state = interface::state::AppState {
        tenant_id,
        client_id,
        client_secret,
        asc_url,
        sender,
        reply_to,
        display_name,
        workspace_id,
        jwt_verifier: jwt_verifier.clone(),
    };

    let port: u16 = port.parse().unwrap();
    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(
                actix_web::web::Data::new(state.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(interface::api::api_handler::receive_alert)
            .service(interface::api::api_handler::receive_alert_secure)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
