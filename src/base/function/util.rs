use crate::base::function::{ConcurrentFn, Curry, Curryed2Fn, WrappedFn};
use crate::base::value::Value;

pub fn id<A>() -> WrappedFn<A, A> {
    WrappedFn::from(|x| x)
}

pub fn compose<A, B, C, F1, F2>(f1: F1, f2: F2) -> WrappedFn<A, C>
where
    A: 'static,
    B: 'static,
    C: 'static,
    F1: Into<WrappedFn<B, C>>,
    F2: Into<WrappedFn<A, B>>,
{
    let (f1, f2) = (f1.into(), f2.into());
    WrappedFn::from(move |x| f1.call(f2.call(x)))
}

pub fn flip<A, B, C, F>(f: F) -> Curryed2Fn<B, A, C>
where
    A: Value + 'static,
    B: Value + 'static,
    C: Value + 'static,
    F: Into<Curryed2Fn<A, B, C>>,
{
    let f = f.into();
    WrappedFn::curry(move |b, a| f(a)(b))
}

pub fn constv<A, B>(x: A) -> WrappedFn<B, A>
where
    A: Value + 'static,
{
    WrappedFn::from(move |_| x.clone())
}

pub fn on<A, B, C, F, G>(f: F, g: G) -> Curryed2Fn<A, A, C>
where
    A: Value + 'static,
    B: Value + 'static,
    C: Value + 'static,
    F: Into<Curryed2Fn<B, B, C>>,
    G: Into<WrappedFn<A, B>>,
{
    let (f, g) = (f.into(), g.into());
    WrappedFn::curry(move |x, y| f(g(x))(g(y)))
}

pub fn curry<A, B, C, F>(f: F) -> Curryed2Fn<A, B, C>
where
    A: Value + 'static,
    B: Value + 'static,
    C: Value + 'static,
    F: Into<WrappedFn<(A, B), C>>,
{
    let f = f.into();
    WrappedFn::curry(move |x, y| f((x, y)))
}

pub fn uncurry<A, B, C, F>(f: F) -> WrappedFn<(A, B), C>
where
    A: Value + 'static,
    B: Value + 'static,
    C: Value + 'static,
    F: Into<Curryed2Fn<A, B, C>>,
{
    let f = f.into();
    WrappedFn::from(move |(x, y)| f(x)(y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let identity = id::<i32>();
        assert_eq!(identity(42), 42);
        assert_eq!(identity(0), 0);
        assert_eq!(identity(-7), -7);
    }

    #[test]
    fn test_compose() {
        let inc_then_double = compose(|x| x * 2, |x| x + 1);
        assert_eq!(inc_then_double(3), 8);

        let len_of_str = compose(|s: String| s.len(), |x: i32| x.to_string());
        assert_eq!(len_of_str(12345), 5);
    }

    #[test]
    fn test_compose_identity() {
        let left = compose(|x| x + 1, id());
        let right = compose(id(), |x| x + 1);
        assert_eq!(left(5), 6);
        assert_eq!(right(5), 6);
    }

    #[test]
    fn test_flip() {
        let sub = WrappedFn::curry(|a, b| a - b);
        let flipped = flip(sub.clone());
        assert_eq!(flipped(3)(10), 7);
        assert_eq!(sub(10)(3), 7);
    }

    #[test]
    fn test_constv() {
        let always_42 = constv(42i32);
        assert_eq!(always_42(1), 42);
        assert_eq!(always_42(999), 42);
        assert_eq!(always_42(-1), 42);
    }

    #[test]
    fn test_on() {
        let diff = WrappedFn::curry(|a, b| a - b);
        let diff_by_len = on(diff, |s: &str| s.len() as i32);
        assert_eq!(diff_by_len("aaa")("b"), 2);
        assert_eq!(diff_by_len("a")("aa"), -1);
        assert_eq!(diff_by_len("ab")("cd"), 0);
    }

    #[test]
    fn test_curry() {
        let add_curried = curry(|(a, b)| a + b);
        assert_eq!(add_curried(3)(4), 7);

        let mul_curried = curry(|(a, b)| a * b);
        assert_eq!(mul_curried(5)(6), 30);
    }

    #[test]
    fn test_uncurry() {
        let div_tuple = uncurry(WrappedFn::curry(|a, b| a / b));
        assert_eq!(div_tuple((10, 3)), 3);
    }

    #[test]
    fn test_curry_uncurry_roundtrip() {
        let add = WrappedFn::curry(|a, b| a + b);
        let add_uncurried = uncurry(add);
        let add_curried_again = curry(add_uncurried.clone());
        assert_eq!(add_curried_again(7)(8), 15);
        assert_eq!(add_uncurried((7, 8)), 15);
    }
}
