pub trait Concurrent: Send + Sync {}

impl<T> Concurrent for T where T: Send + Sync + ?Sized {}

pub trait StaticConcurrent: Concurrent + 'static {}

impl<T> StaticConcurrent for T where T: Concurrent + ?Sized + 'static {}

pub trait Value: Concurrent + Clone {
    type Unwrapped: Concurrent + ?Sized;

    type View<'a>: Concurrent + 'a
    where
        Self: 'a;

    fn make<U>(unwrapped: U) -> Self
    where
        U: Into<Self::Unwrapped>,
        Self::Unwrapped: Sized;

    fn view(&self) -> Self::View<'_>;
}
