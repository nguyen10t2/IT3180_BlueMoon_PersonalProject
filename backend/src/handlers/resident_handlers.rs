use actix_web::{
    HttpResponse, Responder, get, web::{Data, Path, Query}
};

use sqlx::PgPool;

use crate::db::PaginationQuery;
use crate::models::resident::Resident;
use serde_json::json;

#[get("/residents")]
pub async fn get_residents(db: Data<PgPool>, query: Query<PaginationQuery>) -> impl Responder {
    let limit = query.limit.unwrap_or(10);
    let offset = query.offset.unwrap_or(0);

    let result = sqlx::query_as::<_, Resident> (
        "SELECT * FROM residents ORDER BY fullname LIMIT $1 OFFSET $2"
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(residents) => HttpResponse::Ok().json(residents),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}   
#[get("/residents/{id}")]
pub async fn get_resident_by_id(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let resident_id = path.into_inner();

    let result = sqlx::query_as::<_, Resident> (
        "SELECT * FROM residents WHERE resident_id = $1"
    )
        .bind(resident_id)
        .fetch_optional(db.get_ref())
        .await;

    match result {
        Ok(Some(resident)) => HttpResponse::Ok().json(resident),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "Resident not found"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}
