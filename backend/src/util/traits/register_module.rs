use actix_web::Scope;

pub trait RegisterModule<U> {
    fn module(self, func: fn(app: U) -> U) -> Self;
}

impl RegisterModule<Scope> for Scope {
    fn module(self, func: fn(app: Scope) -> Scope) -> Self {
        func(self)
    }
}
