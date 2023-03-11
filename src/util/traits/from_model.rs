pub trait FromModel<T> {
    fn from_model(model: T) -> Self;
}
