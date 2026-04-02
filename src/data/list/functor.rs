use crate::base::value::Value;
use crate::control::functor::Functor;
use crate::data::list::{List, ListInstance};
use crate::data::maybe::Maybe;

impl Functor for ListInstance {
    fn fmap<A, B, G>(g: G, xs: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: Fn(A) -> B>,
    {
        match xs.decompose() {
            Maybe::Just((x, xs)) => {
                let y = (g.view())(x);
                List::cons(y, Self::fmap(g, xs))
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
    fn test_fmap() {
        let xs: List<i32> = List::empty();
        let ys = ListInstance::fmap(&|x| x + 1, xs);
        assert_eq!(ys, List::empty());

        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        let ys = ListInstance::fmap(&|x| x + 1, xs);
        let expected = List::cons(2, List::cons(3, List::cons(4, List::empty())));
        assert_eq!(ys, expected);
    }

    #[test]
    fn test_functor_identity_law() {
        let id = |x| x;

        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        assert_eq!(ListInstance::fmap(arc(id), xs.clone()), xs);

        let xs: List<i32> = List::empty();
        assert_eq!(ListInstance::fmap(arc(id), xs), List::empty());
    }

    #[test]
    fn test_functor_composition_law() {
        let h = |x| (x as i64) * 2;
        let g = |x| x + 3;
        let composed = move |x| g(h(x));

        let xs = List::cons(1, List::cons(2, List::empty()));
        let lhs = ListInstance::fmap(arc(composed), xs.clone());
        let rhs = ListInstance::fmap(arc(g), ListInstance::fmap(arc(h), xs));
        assert_eq!(lhs, rhs);

        let xs: List<i32> = List::empty();
        let lhs = ListInstance::fmap(arc(composed), xs.clone());
        let rhs = ListInstance::fmap(arc(g), ListInstance::fmap(arc(h), xs));
        assert_eq!(lhs, rhs);
    }
}
