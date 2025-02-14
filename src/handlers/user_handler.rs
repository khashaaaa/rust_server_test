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

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
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

pub async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    match sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
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
        user.name.trim(),
        user.email.trim().to_lowercase()
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(new_user) => HttpResponse::Created().json(new_user),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    update: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let current_user = match sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::NotFound().body("User not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Database error"),
    };

    let name = update.name.as_deref().unwrap_or(&current_user.name).trim();
    let email = update.email.as_deref().unwrap_or(&current_user.email).trim().to_lowercase();

    if name.is_empty() || email.is_empty() {
        return HttpResponse::BadRequest().body("Name and email cannot be empty");
    }

    match sqlx::query_as!(
        User,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email",
        name,
        email,
        current_user.id
    )
    .fetch_one(pool.get_ref())
    .await
    {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}

pub async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    match sqlx::query!("DELETE FROM users WHERE id = $1", user_id.into_inner())
        .execute(pool.get_ref())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => HttpResponse::Ok().body("User deleted"),
        Ok(_) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Database error"),
    }
}