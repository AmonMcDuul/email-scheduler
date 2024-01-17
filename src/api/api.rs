use crate::{models::message::Message, repository::database::Database};
use actix_web::web;
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpResponse,
};

#[utoipa::path(
    get,
    path = "/messages",
    responses(
        (status = 200, description = "messages found successfully", body = Message),
        (status = NOT_FOUND, description = "No messages found")
    ),
    // params(
    //     ("id" = u64, Path, description = "id to get message for"),
    // )
)]
#[get("/messages")]
pub async fn get_messages(db: web::Data<Database>) -> HttpResponse {
    let messages = db.get_messages();
    HttpResponse::Ok().json(messages)
}

#[get("/message/{id}")]
pub async fn get_message_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let message = db.get_message_by_id(&id);
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

#[post("/message")]
pub async fn create_message(db: Data<Database>, new_message: Json<Message>) -> HttpResponse {
    let message = db.create_message(new_message.into_inner());
    match message {
        Ok(message) => HttpResponse::Ok().json(message),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/message/{id}")]
pub async fn update_message_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
    updated_message: web::Json<Message>,
) -> HttpResponse {
    let message = db.update_message_by_id(&id, updated_message.into_inner());
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

#[delete("/messages/{id}")]
pub async fn delete_message_by_id(db: web::Data<Database>, id: web::Path<String>) -> HttpResponse {
    let message = db.delete_message_by_id(&id);
    match message {
        Some(message) => HttpResponse::Ok().json(message),
        None => HttpResponse::NotFound().body("Message not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_message)
            .service(get_messages)
            .service(get_message_by_id)
            .service(update_message_by_id)
            .service(delete_message_by_id),
    );
}
