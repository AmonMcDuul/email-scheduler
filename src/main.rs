use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use std::time::Duration;
use tokio::time::interval;

mod api;
mod models;
mod repository;
mod services;

use crate::services::email_service::send_scheduled_emails;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

async fn send_scheduled_emails_periodically(database: web::Data<repository::database::Database>) {
    let mut interval = interval(Duration::from_secs(1 * 10));
    loop {
        interval.tick().await;
        let messages = database.get_messages();
        send_scheduled_emails(&database, messages);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let message_db = repository::database::Database::new();
    let app_data = web::Data::new(message_db);

    tokio::spawn(send_scheduled_emails_periodically(app_data.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
