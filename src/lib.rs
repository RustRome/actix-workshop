use actix_web::web;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel;

#[allow(unused_imports)]
#[macro_use]
extern crate diesel_migrations;

pub mod middleware;
mod test;
pub mod util;

pub struct State {}

pub fn create_state() -> State {
    State {}
}

#[allow(unused_variables)]
pub fn config_app(cfg: &mut web::ServiceConfig) {}
