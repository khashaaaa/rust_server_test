use actix_web::web;
use crate::handlers::user_handler::{create_user, get_users};

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::get().to(get_users))
            .route("/", web::post().to(create_user)),
    );
}
