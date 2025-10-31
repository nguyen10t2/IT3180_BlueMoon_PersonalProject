use actix_web::web;

use crate::handlers::auth_handlers::{
    register_user, login_user, logout_user, change_password, get_current_user
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .service(register_user)
            .service(login_user)
            .service(logout_user)
            .service(change_password)
            .service(get_current_user)
    );
}