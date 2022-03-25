//dependencies
use actix_web::{web,App,HttpServer, Error};
use diesel::prelude::*;
use diesel::r2d2::{self,ConnectionManager};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web::dev::ServiceRequest;
//use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
//use chrono::{Duration,Utc};

//crates
#[macro_use]
extern crate diesel;

//our modules
mod handler;
mod errors;
mod model;
mod schema;
mod auth;




//TODO : Comment useful parts for an upcoming modif
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

//our main function 
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("URL must be set");

    //connexion 
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    // Start http server
    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
            .route("/users", web::get().to(handler::get_users))
            .route("/users/{id}", web::get().to(handler::get_user_by_id))
            .route("/users", web::post().to(handler::add_user))
            .route("/users/{id}", web::delete().to(handler::delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}