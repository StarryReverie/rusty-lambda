use crate::base::value::{Concurrent, Value};
use crate::control::context::alternative::Alternative;
use crate::data::list::{List, ListInstance};

impl Alternative for ListInstance {
    fn fallback<A>() -> Self::Type<A>
    where
        A: Concurrent,
    {
        List::empty()
    }

    fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
    where
        A: Value,
    {
        one.append(another)
    }
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::structure::functor::Functor;

    use super::*;

    #[test]
    fn test_alternative_left_identity_law() {
        let xs = ListInstance::fchain(List::from(vec![1, 2]))
            .alt(ListInstance::fallback())
            .eval();
        assert_eq!(xs, List::from(vec![1, 2]));

        let xs: List<i32> = ListInstance::ffallback()
            .alt(ListInstance::fallback())
            .eval();
        assert_eq!(xs, List::empty());
    }

    #[test]
    fn test_alternative_right_identity_law() {
        let xs = ListInstance::ffallback().alt(List::from(vec![1, 2])).eval();
        assert_eq!(xs, List::from(vec![1, 2]));

        let xs: List<i32> = ListInstance::ffallback()
            .alt(ListInstance::fallback())
            .eval();
        assert_eq!(xs, List::empty());
    }

    #[test]
    fn test_alternative_associativity_law() {
        let a = List::from(vec![1]);
        let b = List::from(vec![2]);
        let c = List::from(vec![3]);
        let lhs = ListInstance::alt(ListInstance::alt(a.clone(), b.clone()), c.clone());
        let rhs = ListInstance::alt(a, ListInstance::alt(b, c));
        assert_eq!(lhs, rhs);

        let a = List::empty();
        let b = List::from(vec![2]);
        let c = List::from(vec![3]);
        let lhs = ListInstance::alt(ListInstance::alt(a.clone(), b.clone()), c.clone());
        let rhs = ListInstance::alt(a, ListInstance::alt(b, c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_left_distributivity_law() {
        let a = List::from(vec![1, 2]);
        let b = List::from(vec![3, 4]);
        let f = WrappedFn::from(|x| x * 10);

        let lhs = ListInstance::fmap(f.clone(), ListInstance::alt(a.clone(), b.clone()));
        let rhs = ListInstance::alt(
            ListInstance::fmap(f.clone(), a),
            ListInstance::fmap(f.clone(), b),
        );
        assert_eq!(lhs, rhs);

        let a = List::empty();
        let b = List::from(vec![3, 4]);

        let lhs = ListInstance::fmap(f.clone(), ListInstance::alt(a.clone(), b.clone()));
        let rhs = ListInstance::alt(ListInstance::fmap(f.clone(), a), ListInstance::fmap(f, b));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_chained_alt() {
        let xs = ListInstance::fchain(List::from(vec![1, 2]))
            .alt(List::from(vec![3]))
            .alt(ListInstance::fallback())
            .alt(List::from(vec![4, 5]))
            .eval();
        assert_eq!(xs, List::from(vec![1, 2, 3, 4, 5]));
    }
}
