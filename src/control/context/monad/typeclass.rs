use crate::base::function::ConcurrentFn;
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::applicative::Applicative;

pub trait Monad: Applicative {
    fn ret<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Self::pure(x)
    }

    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>;
}

pub trait MonadExt {
    type Wrapped: StaticConcurrent;
    type Instance: Monad<Type<Self::Wrapped> = Self>;

    fn ret(x: Self::Wrapped) -> Self
    where
        Self: Sized,
        Self::Wrapped: Value,
    {
        Self::Instance::ret(x)
    }

    fn bind<B, G>(self, g: G) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        G: for<'a> Value<
            View<'a>: ConcurrentFn<
                Self::Wrapped,
                Output = <Self::Instance as TypeConstructor1>::Type<B>,
            >,
        >,
    {
        Self::Instance::bind::<Self::Wrapped, B, G>(self, g)
    }
}
