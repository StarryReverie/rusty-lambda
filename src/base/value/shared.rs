use std::sync::Arc;

use crate::base::value::{StaticConcurrent, Value};

impl<T> Value for Arc<T>
where
    T: StaticConcurrent + ?Sized,
{
    type Unwrapped = T;

    type View<'a> = &'a T;

    fn make<U>(unwrapped: U) -> Self
    where
        U: Into<Self::Unwrapped>,
        Self::Unwrapped: Sized,
    {
        Arc::new(unwrapped.into())
    }

    fn view(&self) -> Self::View<'_> {
        self.as_ref()
    }
}
