use actix_web::{
    HttpResponse, Responder, web::{Data, Path, Query},
    get, delete
};

use sqlx::PgPool;

use crate::db::PaginationQuery;
use crate::models::resident::Resident;
use serde_json::json;

#[get("/")]
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
#[get("/{id}")]
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
#[get("/{name}")]
pub async fn get_resident_by_name(db: Data<PgPool>, path: Path<String>) -> impl Responder {
    let resident_name = path.into_inner();

    let result = sqlx::query_as::<_, Resident> (
        "SELECT * FROM residents WHERE fullname ILIKE $1"
    )
        .bind(resident_name)
        .fetch_all(db.get_ref())
        .await;

    match result {
        Ok(residents) => HttpResponse::Ok().json(residents),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}

#[delete("/{id}")]
pub async fn delete_resident_by_id(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let resident_id = path.into_inner();

    let result = sqlx::query (
        "DELETE FROM residents WHERE resident_id = $1"
    )
        .bind(resident_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "Resident deleted successfully"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}
