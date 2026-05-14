pub mod auth;
pub mod health;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check — used by Nginx and monitoring
        .service(
            web::scope("/health")
                .configure(health::configure)
        )
        // Auth routes — Discord OAuth flow
        .service(
            web::scope("/auth")
                .configure(auth::configure)
        )
        // API routes — all return JSON
        .service(
            web::scope("/api/v1")
                .configure(api_v1)
        )
        // Serve the frontend — catch-all for SPA routing
        .service(
            actix_files::Files::new("/", "./frontend/dist")
                .index_file("index.html")
                .use_last_modified(true)
        );
}

fn api_v1(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/me", web::get().to(auth::me));
}
