use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::structure::functor::Functor;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};

impl Functor for IdentityInstance {
    fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        Identity(g.view().call(x.0))
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;

    use super::*;

    #[test]
    fn test_fmap() {
        let x = Identity(3);
        assert_eq!(
            IdentityInstance::fmap(WrappedFn::from(|x| x + 1), x),
            Identity(4)
        );
    }

    #[test]
    fn test_functor_identity_law() {
        let x = Identity(42);
        assert_eq!(
            IdentityInstance::fmap(WrappedFn::from(|x| x), x),
            Identity(42)
        );
    }

    #[test]
    fn test_functor_composition_law() {
        let h = WrappedFn::from(|x| (x as i64) * 2);
        let g = WrappedFn::from(|x| x + 3);
        let composed = g.clone().compose(h.clone());

        let x = Identity(4i32);
        let lhs = IdentityInstance::fmap(composed.clone(), x);
        let rhs = IdentityInstance::fmap(g, IdentityInstance::fmap(h, Identity(4i32)));
        assert_eq!(lhs, Identity(11i64));
        assert_eq!(lhs, rhs);
    }
}
