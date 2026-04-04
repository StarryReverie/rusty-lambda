use crate::base::hkt::TypeConstructor1;
use crate::base::value::Concurrent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Last<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LastInstance;

impl TypeConstructor1 for LastInstance {
    type Type<A1>
        = Last<A1>
    where
        A1: Concurrent;
}
