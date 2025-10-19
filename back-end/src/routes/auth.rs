use actix_web::{post, web, HttpResponse, Responder};
use crate::models::user::LoginData;

#[post("/login")]
pub async fn login(info: web::Json<LoginData>) -> impl Responder {
    if info.username == "admin" && info.password == "1234" {
        HttpResponse::Ok().json({
            serde_json::json!({
                "status": "success",
                "message": "Đăng nhập thành công!"
            })
        })
    } else {
        HttpResponse::Unauthorized().json({
            serde_json::json!({
                "status": "error",
                "message": "Sai tên đăng nhập hoặc mật khẩu!"
            })
        })
    }
}
