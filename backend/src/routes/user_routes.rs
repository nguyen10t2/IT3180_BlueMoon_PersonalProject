use actix_web::web;

use crate::handlers::user_handlers::{
    get_all_users, get_user_by_id, delete_user, update_user, active_user
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(get_all_users)
            .service(get_user_by_id)
            .service(update_user)
            .service(delete_user)
            .service(active_user)
    );
}

