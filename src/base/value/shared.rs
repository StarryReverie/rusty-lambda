use std::sync::Arc;

use crate::base::value::{StaticConcurrent, Value};

impl<T> Value for Arc<T>
where
    T: StaticConcurrent + ?Sized,
{
    type View<'a> = &'a T;

    fn view(&self) -> Self::View<'_> {
        self.as_ref()
    }
}
