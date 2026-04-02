use crate::base::hkt::TypeConstructor1;
use crate::base::value::Value;

pub trait Functor: TypeConstructor1 {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: Fn(A) -> B>;
}
