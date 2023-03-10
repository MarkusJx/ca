#[macro_export]
macro_rules! register_methods {
    ($($method: ident), *) => {
        pub fn module<T>(mut app: App<T>) -> App<T>
        where
            T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
        {
            $(
                app = app.service($method);
            )+
            app
        }
    };
}
