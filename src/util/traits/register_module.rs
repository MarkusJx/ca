use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::{App, Error};

pub trait RegisterModule<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    fn module(self, func: fn(app: App<T>) -> App<T>) -> Self;
}

impl<T> RegisterModule<T> for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    fn module(self, func: fn(app: App<T>) -> App<T>) -> Self {
        func(self)
    }
}
