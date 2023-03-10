#[macro_export]
macro_rules! register_module {
    ($scope: literal, $($method: ident), *) => {
        pub fn module<T>(app: paperclip::actix::App<T>) -> paperclip::actix::App<T>
        where
            T: actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Error = Error, InitError = ()>,
        {
            let mut scope = paperclip::actix::web::scope($scope);
            $(
                scope = scope.service($method);
            )+

            app.service(scope)
        }
    };
    ($($method: ident), *) => {
        pub fn module<T>(mut app: paperclip::actix::App<T>) -> paperclip::actix::App<T>
        where
            T: actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Error = Error, InitError = ()>,
        {
            $(
                app = app.service($method);
            )+

            app
        }
    }
}
