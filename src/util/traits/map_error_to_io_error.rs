pub trait ErrorToIoError {
    fn to_io_error(self) -> std::io::Error;
}