use crate::db;
use crate::models::{CreatePostRequest, UpdatePostRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/health")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[get("/posts")]
pub async fn get_posts(db_pool: web::Data<PgPool>) -> impl Responder {
    match db::get_all_posts(&db_pool).await {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(e) => {
            log::error!("Failed to get posts: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get posts"
            }))
        }
    }
}

#[get("/posts/{id}")]
pub async fn get_post(db_pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    match db::get_post_by_id(&db_pool, id).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })),
        Err(e) => {
            log::error!("Failed to get post: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get post"
            }))
        }
    }
}

#[post("/posts")]
pub async fn create_post(
    db_pool: web::Data<PgPool>,
    post_req: web::Json<CreatePostRequest>,
) -> impl Responder {
    match db::create_post(&db_pool, post_req.into_inner()).await {
        Ok(post) => HttpResponse::Created().json(post),
        Err(e) => {
            log::error!("Failed to create post: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create post"
            }))
        }
    }
}

#[put("/posts/{id}")]
pub async fn update_post(
    db_pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    post_req: web::Json<UpdatePostRequest>,
) -> impl Responder {
    let id = path.into_inner();

    match db::update_post(&db_pool, id, post_req.into_inner()).await {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })),
        Err(e) => {
            log::error!("Failed to update post: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update post"
            }))
        }
    }
}

#[delete("/posts/{id}")]
pub async fn delete_post(db_pool: web::Data<PgPool>, path: web::Path<Uuid>) -> impl Responder {
    let id = path.into_inner();

    match db::delete_post(&db_pool, id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(serde_json::json!({
            "error": "Post not found"
        })),
        Err(e) => {
            log::error!("Failed to delete post: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete post"
            }))
        }
    }
}
