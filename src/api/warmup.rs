use crate::State;
use actix_web::web::{Data, HttpResponse, Json, Path};
use actix_web::{http::StatusCode, ResponseError};
use serde::{Deserialize, Serialize};


pub fn index() -> String {
    String::from("Hello RustLab")
}
use futures::{future, Future};

pub fn param(path: Path<String>) -> String {
    format!("Hello {}", path)
}

#[derive(Serialize)]
struct Resp {
    message: String,
}
pub fn param_json(path: Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(Resp {
        message: format!("Hello {}", path.into_inner()),
    })
}

#[derive(Deserialize)]
pub struct Body {
    name: String,
}
pub fn json_body(payload: Json<Body>) -> HttpResponse {
    HttpResponse::Ok().json(Resp {
        message: format!("Hello {}", payload.name),
    })
}


#[derive(Serialize, Debug)]
pub struct Error {
    msg: String,
    status: u16,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
impl ResponseError for Error {
    fn render_response(&self) -> HttpResponse {
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap()).json(self)
    }
}

pub fn async_error() -> impl Future<Item = HttpResponse, Error = Error> {
    future::err(Error {
        msg: String::from("some error"),
        status: 400,
    })
}

#[derive(Serialize, Debug)]
pub struct Counter {
    count: i64,
}

pub fn get_requests(state: Data<State>) -> HttpResponse {
    match state.counter.lock() {
        Ok(guard) => HttpResponse::Ok().json(Counter { count: *guard }),
        Err(e) => HttpResponse::InternalServerError().json(Error {
            msg: e.to_string(),
            status: 500,
        }),
    }
}
