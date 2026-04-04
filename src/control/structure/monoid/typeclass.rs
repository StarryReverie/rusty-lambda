use crate::control::structure::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}
