use crate::control::structure::monoid::last::Last;
use crate::control::structure::semigroup::Semigroup;
use crate::data::maybe::Maybe;

impl<T> Semigroup for Last<T> {
    fn associate(self, other: Self) -> Self {
        match (&self.0, &other.0) {
            (_, Maybe::Just(_)) => other,
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(
            Last(Maybe::Just(1)).associate(Last(Maybe::Just(2))),
            Last(Maybe::Just(2))
        );
        assert_eq!(
            Last(Maybe::Just(1)).associate(Last(Maybe::Nothing)),
            Last(Maybe::Just(1))
        );
        assert_eq!(
            Last(Maybe::Nothing).associate(Last(Maybe::Just(2))),
            Last(Maybe::Just(2))
        );
        assert_eq!(
            Last(Maybe::<i32>::Nothing).associate(Last(Maybe::Nothing)),
            Last(Maybe::Nothing)
        );
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = Last(Maybe::Just(1));
        let y = Last(Maybe::Just(2));
        let z = Last(Maybe::Just(3));
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = Last(Maybe::Just(1));
        let y = Last(Maybe::Nothing);
        let z = Last(Maybe::Just(3));
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = Last(Maybe::Nothing);
        let y = Last(Maybe::Just(2));
        let z = Last(Maybe::Nothing);
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = Last(Maybe::<i32>::Nothing);
        let y = Last(Maybe::Nothing);
        let z = Last(Maybe::Nothing);
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);
    }
}
