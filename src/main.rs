use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod db;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    // Set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Create tables if they don't exist
    db::init_db(&db_pool)
        .await
        .expect("Database initialization failed");

    log::info!("Starting server at http://localhost:8100");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::health_check)
                    .service(handlers::get_posts)
                    .service(handlers::get_post)
                    .service(handlers::create_post)
                    .service(handlers::update_post)
                    .service(handlers::delete_post),
            )
    })
    .bind("127.0.0.1:8100")?
    .run()
    .await
}
