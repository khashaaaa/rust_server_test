use actix_web::web;
use crate::handlers::user_handler::{create_user, get_users, get_user, update_user, delete_user};

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("/create", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}/update", web::put().to(update_user))
            .route("/{id}/delete", web::delete().to(delete_user)),
    );
}