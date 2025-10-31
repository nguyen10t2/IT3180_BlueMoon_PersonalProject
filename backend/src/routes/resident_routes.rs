use actix_web::web;

use  crate::handlers::resident_handlers::{get_residents, get_resident_by_id, get_resident_by_name};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("residents")
            .service(get_residents)
            .service(get_resident_by_id)
            .service(get_resident_by_name)
    );
}