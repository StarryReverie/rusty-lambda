use crate::control::structure::monoid::all::All;
use crate::control::structure::semigroup::Semigroup;

impl Semigroup for All {
    fn associate(self, other: Self) -> Self {
        Self(self.0 && other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(All(true).associate(All(true)), All(true));
        assert_eq!(All(true).associate(All(false)), All(false));
        assert_eq!(All(false).associate(All(true)), All(false));
        assert_eq!(All(false).associate(All(false)), All(false));
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = All(true);
        let y = All(false);
        let z = All(true);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = All(false);
        let y = All(false);
        let z = All(false);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = All(true);
        let y = All(true);
        let z = All(true);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));

        let x = All(true);
        let y = All(false);
        let z = All(false);
        assert_eq!(x.associate(y).associate(z), x.associate(y.associate(z)));
    }
}
