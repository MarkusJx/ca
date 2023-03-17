use std::error::Error;

pub trait MapErrorToIoError<T> {
    fn map_to_io_error(self) -> Result<T, std::io::Error>;
}

impl<T> MapErrorToIoError<T> for Result<T, Box<dyn Error>> {
    fn map_to_io_error(self) -> Result<T, std::io::Error> {
        self.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
    }
}
