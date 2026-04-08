use crate::base::function::ConcurrentFn;
use crate::base::hkt::TypeConstructor1;
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::structure::functor::Functor;

pub use crate::control::structure::functor::fmap as lift_a;

pub fn pure<A, FA>(x: A) -> FA
where
    A: Value,
    FA: ApplicativeExt<Wrapped = A> + Value,
{
    FA::Instance::pure(x)
}

pub fn lift_a2<FA, B, C, G2, G1>(
    g: G2,
    x: FA,
    y: <FA::Instance as TypeConstructor1>::Type<B>,
) -> <FA::Instance as TypeConstructor1>::Type<C>
where
    FA: ApplicativeExt<Wrapped: Value> + Value,
    B: Value,
    C: Value,
    G2: for<'a> Value<View<'a>: ConcurrentFn<FA::Wrapped, Output = G1>>,
    G1: for<'a> Value<View<'a>: ConcurrentFn<B, Output = C>>,
{
    let gx = FA::Instance::fmap::<_, _, G2>(g, x);
    FA::Instance::apply::<_, _, G1>(gx, y)
}

pub fn lift_a3<FA, B, C, D, G3, G2, G1>(
    g: G3,
    x: FA,
    y: <FA::Instance as TypeConstructor1>::Type<B>,
    z: <FA::Instance as TypeConstructor1>::Type<C>,
) -> <FA::Instance as TypeConstructor1>::Type<D>
where
    FA: ApplicativeExt<Wrapped: Value> + Value,
    B: Value,
    C: Value,
    D: Value,
    G3: for<'a> Value<View<'a>: ConcurrentFn<FA::Wrapped, Output = G2>>,
    G2: for<'a> Value<View<'a>: ConcurrentFn<B, Output = G1>>,
    G1: for<'a> Value<View<'a>: ConcurrentFn<C, Output = D>>,
{
    let gx = FA::Instance::fmap::<_, _, G3>(g, x);
    let gxy = FA::Instance::apply::<_, _, G2>(gx, y);
    FA::Instance::apply::<_, _, G1>(gxy, z)
}

#[cfg(test)]
mod tests {
    use crate::base::function::{Curry, WrappedFn};
    use crate::data::maybe::Maybe;

    use super::*;

    #[test]
    fn test_lift_a2() {
        let add2 = WrappedFn::curry(|x, y| x + y);
        assert_eq!(
            lift_a2(add2.clone(), Maybe::Just(1), Maybe::Just(2)),
            Maybe::Just(3)
        );
        assert_eq!(
            lift_a2(add2, Maybe::Nothing, Maybe::Just(2)),
            Maybe::Nothing
        );
    }

    #[test]
    fn test_lift_a3() {
        let add3 = WrappedFn::curry(|x, y, z| x + y + z);
        assert_eq!(
            lift_a3(add3.clone(), Maybe::Just(1), Maybe::Just(2), Maybe::Just(3)),
            Maybe::Just(6)
        );
        assert_eq!(
            lift_a3(add3, Maybe::Just(1), Maybe::Nothing, Maybe::Just(3)),
            Maybe::Nothing
        );
    }
}
