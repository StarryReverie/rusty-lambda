use crate::control::structure::monoid::first::First;
use crate::control::structure::semigroup::Semigroup;
use crate::data::maybe::Maybe;

impl<T> Semigroup for First<T> {
    fn associate(self, other: Self) -> Self {
        match (&self.0, &other.0) {
            (Maybe::Just(_), _) => self,
            _ => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associate() {
        assert_eq!(
            First(Maybe::Just(1)).associate(First(Maybe::Just(2))),
            First(Maybe::Just(1))
        );
        assert_eq!(
            First(Maybe::Just(1)).associate(First(Maybe::Nothing)),
            First(Maybe::Just(1))
        );
        assert_eq!(
            First(Maybe::Nothing).associate(First(Maybe::Just(2))),
            First(Maybe::Just(2))
        );
        assert_eq!(
            First(Maybe::<i32>::Nothing).associate(First(Maybe::Nothing)),
            First(Maybe::Nothing)
        );
    }

    #[test]
    fn test_semigroup_associativity_law() {
        let x = First(Maybe::Just(1));
        let y = First(Maybe::Just(2));
        let z = First(Maybe::Just(3));
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = First(Maybe::Just(1));
        let y = First(Maybe::Nothing);
        let z = First(Maybe::Just(3));
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = First(Maybe::Nothing);
        let y = First(Maybe::Just(2));
        let z = First(Maybe::Nothing);
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);

        let x = First(Maybe::<i32>::Nothing);
        let y = First(Maybe::Nothing);
        let z = First(Maybe::Nothing);
        let lhs = x.associate(y).associate(z);
        let rhs = x.associate(y.associate(z));
        assert_eq!(lhs, rhs);
    }
}
