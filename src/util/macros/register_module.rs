#[macro_export]
macro_rules! register_module {
    ($scope: literal, $($method: ident), *) => {
        pub fn register() -> actix_web::Scope {
            let mut scope = actix_web::web::scope($scope);
            $(
                log::debug!(
                    "Registering method '{}' in scope '{}'",
                    stringify!($method),
                    $scope
                );
                scope = scope.service($method);
            )+

            scope
        }
    };
    ($($method: ident), *) => {
        pub fn module(mut app: actix_web::Scope) -> actix_web::Scope {
            $(
                log::debug!(
                    "Registering method '{}'",
                    stringify!($method)
                );
                app = app.service($method);
            )+

            app
        }
    };
}
