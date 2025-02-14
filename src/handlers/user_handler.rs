use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

pub async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    match sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error retrieving users"),
    }
}

pub async fn create_user(
    pool: web::Data<PgPool>,
    user: web::Json<CreateUserRequest>,
) -> impl Responder {
    if user.name.trim().is_empty() || user.email.trim().is_empty() {
        return HttpResponse::BadRequest().body("Name and email cannot be empty");
    }

    match sqlx::query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        user.name,
        user.email
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(new_user) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}
