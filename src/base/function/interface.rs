use crate::base::computation::Computation;
use crate::base::function::WrappedFn;
use crate::base::value::Concurrent;

pub trait ConcurrentFn<T>: Concurrent {
    type Output;

    fn call(&self, argument: T) -> Self::Output;

    fn compose<S, G>(self, other: G) -> WrappedFn<S, Self::Output>
    where
        S: 'static,
        T: 'static,
        Self::Output: 'static,
        Self: Into<WrappedFn<T, Self::Output>>,
        G: Into<WrappedFn<S, T>>,
    {
        let (f, g) = (self.into(), other.into());
        WrappedFn::from(move |x| f(g(x)))
    }
}

impl<T, R, F> ConcurrentFn<T> for F
where
    F: Fn(T) -> R + Concurrent,
{
    type Output = R;

    fn call(&self, argument: T) -> Self::Output {
        self(argument)
    }
}

pub trait ConcurrentTcFn<T>: ConcurrentFn<T, Output = Computation<Self::OutputInner>> {
    type OutputInner;

    fn call_eval(&self, argument: T) -> Self::OutputInner {
        self.call(argument).eval()
    }
}

impl<T, R, F> ConcurrentTcFn<T> for F
where
    F: ConcurrentFn<T, Output = Computation<R>>,
{
    type OutputInner = R;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_fn_compose() {
        let add = |x| x + 1;
        let mul = |x| x * 2;
        let func = add.compose(mul);
        assert_eq!(func(2), 5);
    }
}
