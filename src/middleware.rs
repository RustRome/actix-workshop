use crate::State;

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};

use futures::future::{ok, FutureResult};
use futures::{Future, Poll};


pub struct Counter;

impl<S, B> Transform<S> for Counter
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CounterMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CounterMiddleware { service })
    }
}

pub struct CounterMiddleware<S> {
    service: S,
}

impl<S, B> Service for CounterMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let data = req.app_data::<State>().expect("Cannot get globl state");

        Box::new(self.service.call(req).and_then(move |res| {
            let mut counter = data.counter.lock().expect("Cannot get the lock lock ");

            *counter += 1;
            Ok(res)
        }))
    }
}
