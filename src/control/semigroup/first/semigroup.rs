use crate::control::semigroup::Semigroup;
use crate::control::semigroup::first::First;

impl<T> Semigroup for First<T> {
    fn associate(self, _other: Self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(First(1).associate(First(2)), First(1));
        assert_eq!(First("a").associate(First("b")), First("a"));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs = First(1);
        let ys = First(2);
        let zs = First(3);
        let lhs = xs.associate(ys).associate(zs);
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
