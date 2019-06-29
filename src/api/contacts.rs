use crate::model::{Contact, NewContact};
use crate::State;
use actix_web::{
    web::{self, Data, HttpResponse, Json, Path},
    Error,
};
use futures::Future;


pub fn new_contact(
    contact: Json<NewContact>,
    state: Data<State>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    web::block(move || Contact::create(contact.0, &db.get().expect("Failed to get a connection")))
        .from_err()
        .and_then(|res| Ok(HttpResponse::Created().json(res)))
}

pub fn get_contact(
    path: Path<i32>,
    state: Data<State>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();
    let id = path.into_inner();

    web::block(move || Contact::get_contact(id, &db.get().expect("Failed to get a connection")))
        .from_err()
        .and_then(|res| match res {
            Some(entity) => Ok(HttpResponse::Ok().json(entity)),
            None => Err(Error::from(HttpResponse::NotFound().finish())),
        })
}

pub fn list_contacts(state: Data<State>) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    web::block(move || Contact::list(&db.get().expect("Failed to get a connection")))
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

pub fn delete_contact(
    path: Path<i32>,
    state: Data<State>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();
    let id = path.into_inner();

    web::block(move || Contact::delete(id, &db.get().expect("Failed to get a connection")))
        .from_err()
        .and_then(|_res| Ok(HttpResponse::NoContent().finish()))
}
