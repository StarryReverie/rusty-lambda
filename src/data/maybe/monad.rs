use crate::base::function::ConcurrentFn;
use crate::base::value::Value;
use crate::control::context::monad::Monad;
use crate::data::maybe::{Maybe, MaybeInstance};

impl Monad for MaybeInstance {
    fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        match x {
            Maybe::Just(x) => g.view().call(x),
            Maybe::Nothing => Maybe::Nothing,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::base::value::arc;

    use super::*;

    #[test]
    fn test_bind() {
        let m = MaybeInstance::bind(Maybe::Just(1), &|x| Maybe::Just(x + 1));
        assert_eq!(m, Maybe::Just(2));

        let m: Maybe<i32> = MaybeInstance::bind(Maybe::Nothing, &|x: i32| Maybe::Just(x + 1));
        assert_eq!(m, Maybe::Nothing);

        let m = MaybeInstance::bind(Maybe::Just(42), &|x: i32| Maybe::Just(arc(x.to_string())));
        assert_eq!(m, Maybe::Just(arc("42".to_string())));
    }

    #[test]
    fn test_monad_left_identity_law() {
        let g = |x| Maybe::Just(x * 2);
        let lhs = MaybeInstance::bind(MaybeInstance::ret(3), arc(g));
        let rhs = g(3);
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_monad_right_identity_law() {
        let m = Maybe::Just(42);
        let lhs = MaybeInstance::bind(m, &MaybeInstance::ret);
        assert_eq!(lhs, Maybe::Just(42));

        let m: Maybe<i32> = Maybe::Nothing;
        let lhs = MaybeInstance::bind(m, &MaybeInstance::ret);
        assert_eq!(lhs, Maybe::Nothing);
    }

    #[test]
    fn test_monad_associativity_law() {
        let g = |x| Maybe::Just(x + 1);
        let h = |x| Maybe::Just(x * 2);

        let m = Maybe::Just(3);
        let lhs = MaybeInstance::bind(MaybeInstance::bind(m, arc(g)), arc(h));
        let rhs = MaybeInstance::bind(m, arc(move |x| MaybeInstance::bind(g(x), arc(h))));
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Maybe::Just(8));

        let m: Maybe<i32> = Maybe::Nothing;
        let lhs = MaybeInstance::bind(MaybeInstance::bind(m, arc(g)), arc(h));
        let rhs = MaybeInstance::bind(m, arc(move |x| MaybeInstance::bind(g(x), arc(h))));
        assert_eq!(lhs, rhs);
        assert_eq!(lhs, Maybe::Nothing);
    }

    #[test]
    fn test_chained_bind() {
        let m = MaybeInstance::mreturn(1)
            .bind(&|x| Maybe::Just(x + 1))
            .bind(&|x| Maybe::Just(x * 3))
            .eval();
        assert_eq!(m, Maybe::Just(6));

        let m = MaybeInstance::mchain(Maybe::Nothing)
            .bind(&|x: i32| Maybe::Just(x + 1))
            .eval();
        assert_eq!(m, Maybe::Nothing);

        let m = MaybeInstance::mchain(Maybe::Just(10))
            .bind(&|x| Maybe::Just(x + 5))
            .bind(&|x| {
                if x > 12 {
                    Maybe::Just(x)
                } else {
                    Maybe::Nothing
                }
            })
            .eval();
        assert_eq!(m, Maybe::Just(15));
    }
}
