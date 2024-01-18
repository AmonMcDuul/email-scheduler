use crate::{models::message::Message, repository::database::Database};
use actix_web::web;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};

#[utoipa::path(
    get,
    path = "/api/messages",
    responses(
        (status = 200, description = "Messages found successfully", body = Message),
        (status = NOT_FOUND, description = "No messages found")
    ),
)]
#[get("/messages")]
pub async fn get_messages(db: web::Data<Database>) -> HttpResponse {
    let messages = db.get_messages();
    HttpResponse::Ok().json(messages)
}

#[utoipa::path(
    get,
    path = "/api/message/{id}",
    responses(
        (status = 200, description = "Message found successfully", body = Message),
        (status = NOT_FOUND, description = "Message not found")
    ),
    params(
        ("id" = String, Path, description = "Id to get message for"),
    )
)]
#[get("/message/{id}")]
pub async fn get_message(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let message: Option<Message> = db.get_message(&id);
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

#[utoipa::path(
    post,
    path = "/api/message",
    responses(
        (status = 200, description = "Messages posted successfully", body = Message),
        (status = INTERNAL_SERVER_ERROR, description = "Error posting message")
    ),
)]
#[post("/message")]
pub async fn create_message(db: Data<Database>, new_message: Json<Message>) -> HttpResponse {
    let message = db.create_message(new_message.into_inner());
    match message {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/api/message/{id}",
    responses(
        (status = 200, description = "Message updated successfully", body = Message),
        (status = NOT_FOUND, description = "Message not found")
    ),
    params(
        ("id" = String, Path, description = "Id to get message for"),
    )
)]
#[put("/message/{id}")]
pub async fn update_message(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_message: web::Json<Message>,
) -> HttpResponse {
    let message = db.update_message(&id, updated_message.into_inner());
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

#[utoipa::path(
    delete,
    path = "/api/message/{id}",
    responses(
        (status = 200, description = "Message deleted successfully", body = Message),
        (status = NOT_FOUND, description = "Message not found")
    ),
    params(
        ("id" = String, Path, description = "Id to get message for"),
    )
)]
#[delete("/message/{id}")]
pub async fn delete_message(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let message = db.delete_message(&id);
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_messages)
            .service(get_message)
            .service(create_message)
            .service(update_message)
            .service(delete_message),
    );
}
