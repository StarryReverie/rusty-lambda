use crate::control::structure::semigroup::Semigroup;
use crate::control::structure::semigroup::last::Last;

impl<T> Semigroup for Last<T> {
    fn associate(self, other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(Last(1).associate(Last(2)), Last(2));
        assert_eq!(Last("a").associate(Last("b")), Last("b"));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let xs = Last(1);
        let ys = Last(2);
        let zs = Last(3);
        let lhs = xs.associate(ys).associate(zs);
        let rhs = xs.associate(ys.associate(zs));
        assert_eq!(lhs, rhs);
    }
}
