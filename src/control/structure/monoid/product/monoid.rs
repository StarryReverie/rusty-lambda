use crate::base::numeric::Multiplicative;
use crate::control::structure::monoid::Monoid;
use crate::control::structure::monoid::product::Product;

impl<T> Monoid for Product<T>
where
    T: Multiplicative,
{
    fn empty() -> Self {
        Product(T::one())
    }
}

#[cfg(test)]
mod tests {
    use crate::control::structure::semigroup::Semigroup;

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(Product::empty(), Product(1));
    }

    #[test]
    fn test_monoid_left_identity_law() {
        let xs = Product(5);
        assert_eq!(Product::empty().associate(xs), xs);
    }

    #[test]
    fn test_monoid_right_identity_law() {
        let xs = Product(5);
        assert_eq!(xs.associate(Product::empty()), xs);
    }
}
