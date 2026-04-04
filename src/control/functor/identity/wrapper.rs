use crate::base::hkt::TypeConstructor1;
use crate::base::value::Concurrent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Identity<T>(pub T);

impl<T> Identity<T> {
    pub fn run(s: Self) -> T {
        s.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IdentityInstance;

impl TypeConstructor1 for IdentityInstance {
    type Type<A1>
        = Identity<A1>
    where
        A1: Concurrent;
}
