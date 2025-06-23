
use crate::models::{CreateTask, Task};
use actix_web::{HttpResponse, delete, get, post, put, web};
use sqlx::PgPool;

#[get("/tasks")]
pub async fn get_tasks(pool: web::Data<PgPool>) -> HttpResponse {
    let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
        .fetch_all(pool.get_ref())
        .await;

    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/tasks")]
pub async fn add_task(pool: web::Data<PgPool>, body: web::Json<CreateTask>) -> HttpResponse {
    let result = sqlx::query("INSERT INTO tasks (title, completed) VALUES ($1, false)")
        .bind(&body.title)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/tasks/{id}")]
pub async fn complete_task(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    let result = sqlx::query("UPDATE tasks SET completed = true WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/tasks/{id}")]
pub async fn delete_task(pool: web::Data<PgPool>, id: web::Path<i32>) -> HttpResponse {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
        .bind(*id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}