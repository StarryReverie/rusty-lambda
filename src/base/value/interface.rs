pub trait Concurrent: Send + Sync {}

impl<T> Concurrent for T where T: Send + Sync + ?Sized {}

pub trait StaticConcurrent: Concurrent + 'static {}

impl<T> StaticConcurrent for T where T: Concurrent + ?Sized + 'static {}

pub trait Value: Concurrent + Clone {
    type View<'a>: Concurrent + 'a
    where
        Self: 'a;

    fn view(&self) -> Self::View<'_>;
}
