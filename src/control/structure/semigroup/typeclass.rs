pub trait Semigroup {
    fn associate(self, other: Self) -> Self;
}
