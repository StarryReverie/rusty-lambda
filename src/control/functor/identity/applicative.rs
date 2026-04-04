use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::applicative::Applicative;
use crate::control::functor::identity::{Identity, IdentityInstance};

impl Applicative for IdentityInstance {
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        Identity(x)
    }

    fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Identity(g.0.view().call(x.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;

    use super::*;

    #[test]
    fn test_pure() {
        assert_eq!(IdentityInstance::pure(42), Identity(42));
    }

    #[test]
    fn test_apply() {
        assert_eq!(
            IdentityInstance::apply(Identity(WrappedFn::from(|x: i32| x + 1)), Identity(1)),
            Identity(2),
        );
    }

    #[test]
    fn test_applicative_identity_law() {
        let x = Identity(42);
        assert_eq!(
            IdentityInstance::apply(IdentityInstance::pure(WrappedFn::from(|x| x)), x),
            Identity(42),
        );
    }

    #[test]
    fn test_applicative_homomorphism_law() {
        let h = WrappedFn::from(|x| x * 2);
        assert_eq!(
            IdentityInstance::apply(IdentityInstance::pure(h.clone()), IdentityInstance::pure(3)),
            IdentityInstance::pure(h(3)),
        );
    }

    #[test]
    fn test_applicative_interchange_law() {
        let h = WrappedFn::from(|x| x + 10);
        let x = 5;

        let lhs = IdentityInstance::apply(Identity(h.clone()), IdentityInstance::pure(x));
        let rhs = IdentityInstance::apply(
            IdentityInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x))),
            Identity(h),
        );
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_applicative_composition_law() {
        let g = WrappedFn::from(|x| x * 2);
        let h = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let lhs = IdentityInstance::apply(Identity(composed.clone()), Identity(4));
        let rhs = IdentityInstance::apply(
            Identity(g.clone()),
            IdentityInstance::apply(Identity(h.clone()), Identity(4)),
        );
        assert_eq!(lhs, Identity(14));
        assert_eq!(lhs, rhs);
    }
}
