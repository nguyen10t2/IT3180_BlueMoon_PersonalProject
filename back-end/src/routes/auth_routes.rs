use actix_web::web;

use crate::handlers::auth_handlers::{
    register_user, login_user
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("home")
            .service(register_user)
            .service(login_user)
    );
}