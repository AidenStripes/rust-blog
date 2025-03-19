use crate::models::{CreatePostRequest, Post, UpdatePostRequest};
use chrono::Utc;
use sqlx::{postgres::PgPool, Error};
use uuid::Uuid;

pub async fn init_db(pool: &PgPool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id UUID PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            author TEXT NOT NULL,
            published BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL,
            updated_at TIMESTAMPTZ
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_all_posts(pool: &PgPool) -> Result<Vec<Post>, Error> {
    let posts = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, author, published, created_at, updated_at
        FROM posts
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(posts)
}

pub async fn get_post_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Post>, Error> {
    let post = sqlx::query_as!(
        Post,
        r#"
        SELECT id, title, content, author, published, created_at, updated_at
        FROM posts
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn create_post(pool: &PgPool, post_req: CreatePostRequest) -> Result<Post, Error> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let published = post_req.published.unwrap_or(false);

    let post = sqlx::query_as!(
        Post,
        r#"
        INSERT INTO posts (id, title, content, author, published, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, title, content, author, published, created_at, updated_at
        "#,
        id,
        post_req.title,
        post_req.content,
        post_req.author,
        published,
        now,
        None::<chrono::DateTime<Utc>>
    )
    .fetch_one(pool)
    .await?;

    Ok(post)
}

pub async fn update_post(
    pool: &PgPool,
    id: Uuid,
    post_req: UpdatePostRequest,
) -> Result<Option<Post>, Error> {
    // First check if post exists
    let existing = get_post_by_id(pool, id).await?;

    if existing.is_none() {
        return Ok(None);
    }

    let existing = existing.unwrap();
    let now = Utc::now();

    // Update only provided fields
    let title = post_req.title.unwrap_or(existing.title);
    let content = post_req.content.unwrap_or(existing.content);
    let published = post_req.published.unwrap_or(existing.published);

    let post = sqlx::query_as!(
        Post,
        r#"
        UPDATE posts
        SET title = $1, content = $2, published = $3, updated_at = $4
        WHERE id = $5
        RETURNING id, title, content, author, published, created_at, updated_at
        "#,
        title,
        content,
        published,
        now,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(post))
}

pub async fn delete_post(pool: &PgPool, id: Uuid) -> Result<bool, Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM posts
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
