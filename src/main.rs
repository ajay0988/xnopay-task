#[macro_use]
extern crate bson;

use actix_web::{web, App, HttpServer};

use crate::logging::*;

// import the created modules
mod common;
mod logging;
mod resource;
mod db;


// 
#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    init_logger();
    // let binding_address = get_binding_address();

    HttpServer::new(|| App::new()
        .service(
            web::scope("/user")
                .route("", web::get().to(resource::get_all))
                .route("", web::post().to(resource::save))
                .route("{id}", web::get().to(resource::get))
                .route("{id}", web::put().to(resource::update))
                .route("{id}", web::delete().to(resource::delete))
        ))
        .bind("0.0.0.0:4000")
        .expect(&format!("Can not bind to {}", "0.0.0.0:4000") )
        .run()
        .await
}
