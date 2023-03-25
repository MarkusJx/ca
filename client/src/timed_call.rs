use std::future::Future;
use std::time::Duration;

pub struct TimedCall<T> {
    pub duration: Duration,
    pub result: T,
}

impl<T: Sized> TimedCall<T> {
    pub async fn time<F, Fut>(func: F) -> TimedCall<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = T>,
    {
        let start = std::time::Instant::now();
        let result = func().await;
        let duration = start.elapsed();

        TimedCall { duration, result }
    }
}
