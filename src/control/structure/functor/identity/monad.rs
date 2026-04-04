use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::monad::Monad;
use crate::control::structure::functor::identity::IdentityInstance;

impl Monad for IdentityInstance {
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        g.view().call(x.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::structure::functor::identity::Identity;

    use super::*;

    #[test]
    fn test_bind() {
        assert_eq!(
            IdentityInstance::bind(Identity(1), WrappedFn::from(|x| Identity(x + 1))),
            Identity(2),
        );
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = WrappedFn::from(|x| Identity(x * 2));
        assert_eq!(
            IdentityInstance::bind(IdentityInstance::ret(3), g.clone()),
            g(3)
        );
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = Identity(42);
        assert_eq!(
            IdentityInstance::bind(m, WrappedFn::from(|x| IdentityInstance::ret(x))),
            Identity(42),
        );
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = WrappedFn::from(|x| Identity(x + 1));
        let h = WrappedFn::from(|x| Identity(x * 2));

        let m = Identity(3);
        let lhs = IdentityInstance::bind(IdentityInstance::bind(m, g.clone()), h.clone());
        let rhs = IdentityInstance::bind(
            m,
            WrappedFn::from(move |x| IdentityInstance::bind(g(x), h.clone())),
        );
        assert_eq!(lhs, Identity(8));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_chained_bind() {
        let m = IdentityInstance::mreturn(1)
            .bind(WrappedFn::from(|x| Identity(x + 1)))
            .bind(WrappedFn::from(|x| Identity(x * 3)))
            .eval();
        assert_eq!(m, Identity(6));
    }
}
