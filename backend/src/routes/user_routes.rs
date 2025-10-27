use actix_web::web;

use crate::handlers::user_handlers::{
    get_all_users, get_user_by_id
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/users")
            .service(get_all_users)
            .service(get_user_by_id)
    );
}

