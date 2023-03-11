#[macro_export]
macro_rules! register_module {
    ($scope: literal, $($method: ident), *) => {
        pub fn module<T>(app: actix_web::App<T>) -> actix_web::App<T>
        where
            T: actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
        {
            let mut scope = actix_web::web::scope($scope);
            $(
                scope = scope.service($method);
            )+

            app.service(scope)
        }
    };
    ($($method: ident), *) => {
        pub fn module<T>(mut app: actix_web::App<T>) -> actix_web::App<T>
        where
            T: actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
        {
            $(
                app = app.service($method);
            )+

            app
        }
    };
}
