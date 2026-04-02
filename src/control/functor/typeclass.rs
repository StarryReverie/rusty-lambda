use std::borrow::Borrow;

use crate::base::hkt::TypeConstructor1;
use crate::base::value::{StaticConcurrent, Value};

pub trait Functor: TypeConstructor1 {
    fn fmap<A, B, G, GI>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: Borrow<GI> + Value,
        GI: Fn(A) -> B + StaticConcurrent;
}
