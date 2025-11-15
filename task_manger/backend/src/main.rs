use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::connect().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::routes)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}