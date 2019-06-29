use actix_files::Files;
use actix_web::web;
use std::sync::Mutex;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel_migrations;

mod util;
mod api;
pub mod middleware;
pub mod model;
pub mod schema;
mod test;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    sqlite::SqliteConnection,
};

pub struct State {
    counter: Mutex<i64>,
    db: Pool<ConnectionManager<SqliteConnection>>,
}

pub fn create_state() -> State {
    State {
        counter: Mutex::new(0),
        db: util::pool(),
    }
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(api::index))
        .service(Files::new("/static", "static").index_file("index.html"))
        .service(web::resource("/hello/{name}").route(web::get().to(api::param)))
        .service(web::resource("/hello_json/{name}").route(web::get().to(api::param_json)))
        .service(web::resource("/json_body").route(web::post().to(api::json_body)))
        .service(web::resource("/async_error").route(web::get().to_async(api::async_error)))
        .service(web::resource("/requests").route(web::get().to(api::get_requests)))
        .service(
            web::scope("/api")
                .service(
                    web::resource("/contacts")
                        .route(web::post().to_async(api::new_contact))
                        .route(web::get().to_async(api::list_contacts)),
                )
                .service(
                    web::resource("/contacts/{id}")
                        .route(web::get().to_async(api::get_contact))
                        .route(web::delete().to_async(api::delete_contact)),
                ),
        );
}
