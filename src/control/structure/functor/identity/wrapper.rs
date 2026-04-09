use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Identity<T>(pub T);

impl<T> Identity<T> {
    pub fn run(s: Self) -> T {
        s.0
    }
}

impl<T> SimpleValue for Identity<T> where T: Value {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IdentityInstance;

impl ContextConstructor for IdentityInstance {
    type Type<A>
        = Identity<A>
    where
        A: Value;
}

crate::derive_functor_for_wrapper!(IdentityInstance, Identity);
crate::derive_foldable_for_wrapper!(IdentityInstance, Identity);
crate::derive_traversable_for_wrapper!(IdentityInstance, Identity);
crate::derive_applicative_for_wrapper!(IdentityInstance, Identity);
crate::derive_monad_for_wrapper!(IdentityInstance, Identity);

crate::derive_semigroup_for_wrapper!(for<T>, Identity<T>);
crate::derive_monoid_for_wrapper!(for<T>, Identity<T>);
