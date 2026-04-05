use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::any::Any;

impl Monoid for Any {
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
        assert_eq!(Any::empty(), Any(false));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = Any(true);
        assert_eq!(Any::empty().associate(xs.clone()), xs);

        let xs = Any(false);
        assert_eq!(Any::empty().associate(xs.clone()), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = Any(true);
        assert_eq!(xs.clone().associate(Any::empty()), xs);

        let xs = Any(false);
        assert_eq!(xs.clone().associate(Any::empty()), xs);
    }
}
