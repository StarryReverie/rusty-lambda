use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::monad::Monad;
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

#[cfg(test)]
mod tests {
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_bind() {
        let xs: List<i32> = List::empty();
        let g = |x| List::cons(x + 1, List::empty());
        let ys = ListInstance::bind(xs, arc(g));
        assert_eq!(ys, List::empty());

        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        let g = |x| List::cons(x + 1, List::empty());
        let ys = ListInstance::bind(xs, arc(g));
        assert_eq!(
            ys,
            List::cons(2, List::cons(3, List::cons(4, List::empty())))
        );

        let xs = List::cons(1, List::cons(2, List::empty()));
        let g = |x| List::cons(x as i64, List::cons(x as i64 * 10, List::empty()));
        let ys = ListInstance::bind(xs, arc(g));
        assert_eq!(
            ys,
            List::cons(
                1i64,
                List::cons(10i64, List::cons(2i64, List::cons(20i64, List::empty())))
            )
        );
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = |x: i32| List::cons(x * 2, List::cons(x * 3, List::empty()));

        let lhs = ListInstance::bind(ListInstance::ret(3), arc(g));
        let rhs = g(3);
        assert_eq!(lhs, rhs);

        let g = |_x: i32| -> List<i32> { List::empty() };
        let lhs = ListInstance::bind(ListInstance::ret(5), arc(g));
        let rhs = g(5);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_monad_right_identity_law() {
        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        let ys = ListInstance::bind(xs.clone(), arc(|x| ListInstance::ret(x)));
        assert_eq!(ys, xs);

        let xs: List<i32> = List::empty();
        let ys = ListInstance::bind(xs.clone(), arc(|x| ListInstance::ret(x)));
        assert_eq!(ys, xs);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = |x| List::cons(x, List::cons(x * 10, List::empty()));
        let h = |x| List::cons(x + 1, List::empty());

        let xs = List::cons(1, List::cons(2, List::empty()));
        let lhs = ListInstance::bind(ListInstance::bind(xs.clone(), arc(g)), arc(h));
        let k = move |x| ListInstance::bind(g(x), arc(h));
        let rhs = ListInstance::bind(xs, arc(k));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let lhs = ListInstance::bind(ListInstance::bind(xs.clone(), arc(g)), arc(h));
        let k = move |x| ListInstance::bind(g(x), arc(h));
        let rhs = ListInstance::bind(xs, arc(k));
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_chained_bind() {
        let xs = ListInstance::mreturn(1)
            .bind(arc(|x| List::cons(x, List::cons(x + 1, List::empty()))))
            .bind(arc(|x| List::cons(x * 10, List::empty())))
            .eval();
        assert_eq!(xs, List::cons(10, List::cons(20i32, List::empty())));

        let xs: List<i32> = List::empty();
        let ys = ListInstance::mchain(xs)
            .bind(arc(|x| List::cons(x, List::empty())))
            .eval();
        assert_eq!(ys, List::empty());
    }
}
