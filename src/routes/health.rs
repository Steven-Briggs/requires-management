use actix_web::{web, HttpResponse};
use central_core::response::ApiResponse;
use serde::Serialize;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("", web::get().to(health));
}

#[derive(Serialize)]
struct HealthResponse {
    service: &'static str,
    version: &'static str,
    status: &'static str,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::ok(HealthResponse {
        service: "requires.management",
        version: env!("CARGO_PKG_VERSION"),
        status: "ok",
    }))
}
