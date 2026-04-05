use crate::control::structure::monoid::any::Any;
use crate::control::structure::semigroup::Semigroup;

impl Semigroup for Any {
    fn associate(self, other: Self) -> Self {
        Self(self.0 || other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(Any(true).associate(Any(true)), Any(true));
        assert_eq!(Any(true).associate(Any(false)), Any(true));
        assert_eq!(Any(false).associate(Any(true)), Any(true));
        assert_eq!(Any(false).associate(Any(false)), Any(false));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = Any(true);
        let y = Any(false);
        let z = Any(true);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Any(false);
        let y = Any(false);
        let z = Any(false);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Any(true);
        let y = Any(true);
        let z = Any(true);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = Any(true);
        let y = Any(false);
        let z = Any(false);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));
    }
}
