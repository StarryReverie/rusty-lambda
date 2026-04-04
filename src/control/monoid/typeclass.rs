use crate::control::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}
