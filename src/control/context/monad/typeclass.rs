use crate::base::function::{ConcurrentFn, constv};
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, Value};
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

    fn then<A, B>(x: Self::Type<A>, y: Self::Type<B>) -> Self::Type<B>
    where
        Self: Sized,
        A: Value,
        B: Value,
        Self::Type<B>: Value,
    {
        Self::bind(x, constv(y))
    }
}

pub trait MonadExt {
    type Wrapped: Concurrent;
    type Instance: Monad<Type<Self::Wrapped> = Self>;

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

    fn then<B>(
        self,
        y: <Self::Instance as TypeConstructor1>::Type<B>,
    ) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        <Self::Instance as TypeConstructor1>::Type<B>: Value,
    {
        Self::Instance::then::<Self::Wrapped, B>(self, y)
    }
}
