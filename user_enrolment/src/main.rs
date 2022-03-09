#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware,web,App,HttpServer,http::header};
use diesel::{prelude::*,r2d2::{self,ConnectionManager}};

//expose our config file to all the project
mod config;
mod model;
mod schema;
mod errors;

#[actix_rt::main]
async fn main() -> std::io::Result<()>
{

    //logging our server info
    std::env::set_var("RUST_LOG", "actix_web= info, actix_server=info");
    env_logger::init();

    //database connection 
    let manager = ConnectionManager::<PgConnection>::new(config::database_url());
    let pool: model::Pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create a database connection pool");

    //httpServer
    HttpServer::new(move || {
        App::new()
        .data(pool.clone())
        .wrap(middleware::Logger::default())//logger
        
        .wrap(CookieSession::signed(&[0;32])
        .domain(config::domain_url().as_str())
        .name("auth")
        .secure(false))

        .wrap(Cors::new()
        .allowed_origin("*")
        .allowed_methods(vec!["GET","POST","DELETE"])
        .max_age(3600)
        .finish())

        .service(Files::new("/assets","./templates/assets"))
    })
    .bind(format!("{}:{}", config::domain(),config::port()))?
    .run()
    .await

}
