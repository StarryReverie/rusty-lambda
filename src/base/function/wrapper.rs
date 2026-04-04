use std::ops::Deref;
use std::sync::Arc;

use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};

pub struct WrappedFn<T, R>(Arc<dyn Fn(T) -> R + Send + Sync + 'static>);

impl<T, R, F> From<F> for WrappedFn<T, R>
where
    F: Fn(T) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        Self(Arc::new(func))
    }
}

impl<T, R> ConcurrentFn<T> for WrappedFn<T, R> {
    type Output = R;

    fn call(&self, argument: T) -> Self::Output {
        (self.0)(argument)
    }
}

impl<T, R> Value for WrappedFn<T, R> {
    type Unwrapped = Self;

    type View<'a>
        = &'a <Self as Deref>::Target
    where
        Self: 'a;

    fn make<U>(unwrapped: U) -> Self
    where
        U: Into<Self::Unwrapped>,
        Self::Unwrapped: Sized,
    {
        unwrapped.into()
    }

    fn view(&self) -> Self::View<'_> {
        &**self
    }
}

impl<T, R> Deref for WrappedFn<T, R> {
    type Target = dyn Fn(T) -> R + Send + Sync;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T, R> Clone for WrappedFn<T, R> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrapped_fn_ref() {
        let inc = |x: &mut i32| {
            *x += 1;
        };
        let mut x = 1;
        {
            let inc = WrappedFn::from(inc);
            inc.call(&mut x);
        }
        assert_eq!(x, 2);
    }
}
