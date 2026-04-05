use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::all::All;

impl Monoid for All {
    fn empty() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::control::structure::semigroup::Semigroup;

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(All::empty(), All(true));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = All(true);
        assert_eq!(All::empty().associate(xs.clone()), xs);

        let xs = All(false);
        assert_eq!(All::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = All(true);
        assert_eq!(xs.clone().associate(All::empty()), xs);

        let xs = All(false);
        assert_eq!(xs.clone().associate(All::empty()), xs);
    }
}
