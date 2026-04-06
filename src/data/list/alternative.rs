use crate::base::value::{Concurrent, StaticConcurrent, Value};
use crate::control::context::alternative::{Alternative, AlternativeExt};
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

impl<T> AlternativeExt for List<T>
where
    T: StaticConcurrent,
{
    type Wrapped = T;
    type Instance = ListInstance;
}

#[cfg(test)]
mod tests {
    use crate::base::function::WrappedFn;
    use crate::control::structure::functor::Functor;

    use super::*;

    #[test]
    fn test_alternative_left_identity_law() {
        let xs = List::from(vec![1, 2]).alt(ListInstance::fallback());
        assert_eq!(xs, List::from(vec![1, 2]));

        let xs: List<i32> = ListInstance::fallback().alt(ListInstance::fallback());
        assert_eq!(xs, List::empty());
    }

    #[test]
    fn test_alternative_right_identity_law() {
        let xs = ListInstance::fallback().alt(List::from(vec![1, 2]));
        assert_eq!(xs, List::from(vec![1, 2]));

        let xs: List<i32> = ListInstance::fallback().alt(ListInstance::fallback());
        assert_eq!(xs, List::empty());
    }

    #[test]
    fn test_alternative_associativity_law() {
        let a = List::from(vec![1]);
        let b = List::from(vec![2]);
        let c = List::from(vec![3]);
        let lhs = a.clone().alt(b.clone()).alt(c.clone());
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);

        let a = List::empty();
        let b = List::from(vec![2]);
        let c = List::from(vec![3]);
        let lhs = a.clone().alt(b.clone()).alt(c.clone());
        let rhs = a.alt(b.alt(c));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_alternative_left_distributivity_law() {
        let a = List::from(vec![1, 2]);
        let b = List::from(vec![3, 4]);
        let f = WrappedFn::from(|x| x * 10);

        let lhs = ListInstance::fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = ListInstance::fmap(f.clone(), a).alt(ListInstance::fmap(f.clone(), b));
        assert_eq!(lhs, rhs);

        let a = List::empty();
        let b = List::from(vec![3, 4]);

        let lhs = ListInstance::fmap(f.clone(), a.clone().alt(b.clone()));
        let rhs = ListInstance::fmap(f.clone(), a).alt(ListInstance::fmap(f, b));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_chained_alt() {
        let xs = List::from(vec![1, 2])
            .alt(List::from(vec![3]))
            .alt(ListInstance::fallback())
            .alt(List::from(vec![4, 5]));
        assert_eq!(xs, List::from(vec![1, 2, 3, 4, 5]));
    }
}
