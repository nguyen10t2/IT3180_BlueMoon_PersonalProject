use actix_web::{
    HttpResponse, Responder, delete, get, put, post, web::{Data, Json, Path}
};

use sqlx::PgPool;

use crate::models::user::User;
use serde_json::json;

#[get("/")]
pub async fn get_all_users(db: Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE status = 'active'"
    )
        .fetch_all(db.get_ref())
        .await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}


#[get("/{id}")]
pub async fn get_user_by_id(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE user_id = $1"
    )
        .bind(user_id)
        .fetch_optional(db.get_ref())
        .await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}

#[put("/update-user/{id}")]
pub async fn update_user(db: Data<PgPool>, path: Path<i32>, user_info: Json<User>) -> impl Responder {
    let user_id = path.into_inner();
    let user = user_info.into_inner();

    let result = sqlx::query(
        "UPDATE users SET username = $1, fullname = $2, email = $3, role = $4 WHERE user_id = $5"
    )
        .bind(user.username)
        .bind(user.fullname)
        .bind(user.email)
        .bind(user.role)
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "User updated successfully"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()})),
    }
}

#[delete("/delete-user/{id}")]
pub async fn delete_user(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query(
        "UPDATE users SET status = 'inactive' WHERE user_id = $1" 
    )
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"message": "Delete user successfully"}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"error": "User not found"})),
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()}))
    }
}

#[post("/active-user/{id}")]
pub async fn active_user(db: Data<PgPool>, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let result = sqlx::query(
        "UPDATE users SET status = 'active' WHERE user_id = $1"
    )
        .bind(user_id)
        .execute(db.get_ref())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                HttpResponse::NotFound().json(json!({"message": "User not found"}))
            } else {
                HttpResponse::Ok().json(json!({"message": "Actived"}))
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(json!({"error": err.to_string()}))
    }
}

