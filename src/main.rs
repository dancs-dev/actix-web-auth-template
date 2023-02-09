use actix_web::{web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;

mod actions;
mod auth;
mod models;
mod schema;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Check we have all the envs we expect:
    std::env::var("DATABASE_URL").expect("DATABASE_URL");
    std::env::var("SECRET_KEY").expect("SECRET_KEY");

    let db_conn_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let db_conn_url = config::db_conn_url;
    let manager = ConnectionManager::<SqliteConnection>::new(db_conn_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(auth::CheckAuth)
            .service(views::index)
            .service(views::signup)
            .service(views::signin)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

