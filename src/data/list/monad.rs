use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::control::context::monad::{Monad, MonadExt};
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

impl Monad for ListInstance {
    fn bind<A, B, G>(xs: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        match xs.decompose() {
            Maybe::Just((x, xs)) => {
                let ys = g.view().call(x);
                ys.append(Self::bind(xs, g))
            }
            Maybe::Nothing => List::empty(),
        }
    }
}

impl<T> MonadExt for List<T>
where
    T: StaticConcurrent,
{
    type Wrapped = T;
    type Instance = ListInstance;
}

#[cfg(test)]
mod tests {
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_bind() {
        let xs: List<i32> = List::empty();
        let g = |x| List::singleton(x + 1);
        let ys = xs.bind(arc(g));
        assert_eq!(ys, List::empty());

        let xs = List::from(vec![1, 2, 3]);
        let g = |x| List::singleton(x + 1);
        let ys = xs.bind(arc(g));
        assert_eq!(ys, List::from(vec![2, 3, 4]));

        let xs = List::from(vec![1, 2]);
        let g = |x| List::from(vec![x as i64, x as i64 * 10]);
        let ys = xs.bind(arc(g));
        assert_eq!(ys, List::from(vec![1i64, 10i64, 2i64, 20i64]));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = |x: i32| List::from(vec![x * 2, x * 3]);

        let lhs = ListInstance::ret(3).bind(arc(g));
        let rhs = g(3);
        assert_eq!(lhs, rhs);

        let g = |_x: i32| -> List<i32> { List::empty() };
        let lhs = ListInstance::ret(5).bind(arc(g));
        let rhs = g(5);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_monad_right_identity_law() {
        let xs = List::from(vec![1, 2, 3]);
        let ys = xs.clone().bind(arc(|x| ListInstance::ret(x)));
        assert_eq!(ys, xs);

        let xs: List<i32> = List::empty();
        let ys = xs.clone().bind(arc(|x| ListInstance::ret(x)));
        assert_eq!(ys, xs);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = |x| List::from(vec![x, x * 10]);
        let h = |x| List::singleton(x + 1);

        let xs = List::from(vec![1, 2]);
        let lhs = xs.clone().bind(arc(g)).bind(arc(h));
        let k = move |x| g(x).bind(arc(h));
        let rhs = xs.bind(arc(k));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let lhs = xs.clone().bind(arc(g)).bind(arc(h));
        let k = move |x| g(x).bind(arc(h));
        let rhs = xs.bind(arc(k));
        assert_eq!(lhs, rhs);
    }
}
