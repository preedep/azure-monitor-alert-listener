mod interface;
mod domain;

use actix_web::HttpServer;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    info!("Starting Azure Monitor Alert Listener on port {}", port);

    let port: u16 = port.parse().unwrap();
    HttpServer::new(|| {
        actix_web::App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(interface::api::api_handler::receive_alert)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
