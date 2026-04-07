use crate::base::function::ConcurrentFn;
use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};

pub trait Functor: TypeConstructor1 {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>;
}

pub trait FunctorExt {
    type Wrapped: StaticConcurrent;
    type Instance: Functor<Type<Self::Wrapped> = Self>;

    fn piped<B, G>(self, g: G) -> <Self::Instance as TypeConstructor1>::Type<B>
    where
        Self: Sized,
        Self::Wrapped: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<Self::Wrapped, Output = B>>,
    {
        Self::Instance::fmap::<Self::Wrapped, B, G>(g, self)
    }
}
