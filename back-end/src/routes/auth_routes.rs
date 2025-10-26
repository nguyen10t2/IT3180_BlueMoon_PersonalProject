use actix_web::web;

use crate::handlers::auth_handlers::{
    register_user
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .service(register_user)
    );
}