use crate::base::function::ConcurrentFn;
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, Value};
use crate::control::structure::functor::Functor;

pub trait Applicative: Functor {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value;

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>;
}

pub trait ApplicativeExt {
    type Wrapped: Concurrent;
    type Instance: Applicative<Type<Self::Wrapped> = Self>;

    fn apply<A, B>(
        self,
        x: <Self::Instance as TypeConstructor1>::Type<A>,
    ) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        A: Value,
        B: Value,
        Self: Sized,
        Self::Wrapped: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Self::Instance::apply::<A, B, Self::Wrapped>(self, x)
    }
}
