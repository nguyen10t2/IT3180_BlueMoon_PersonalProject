use actix_web::web;

use crate::handlers::user_handlers::{
    get_all_users
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(get_all_users)
    );
}

