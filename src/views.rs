use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/signup")]
async fn signup(pool: web::Data<DbPool>, form: web::Json<crate::models::NewUser>) -> Result<HttpResponse, Error> {
    let new_user = web::block(move || {
        let mut conn = pool.get()?;
        actions::create_new_user(&mut conn, &form.username, &form.password)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(new_user))
}

#[post("/signin")]
async fn signin(pool: web::Data<DbPool>, form: web::Json<crate::models::NewUser>) -> Result<HttpResponse, Error>  {
    let new_user = web::block(move || {
        let mut conn = pool.get()?;
        actions::login_user(&mut conn, &form.username, &form.password)
    });
   
    let token = new_user.await.unwrap().unwrap();

    // This should probably be an option, to force handling.
    if token == "" {
        Ok(HttpResponse::Unauthorized().body("Invalid"))
    } else {
        Ok(HttpResponse::Ok().body(token))
    }

}

