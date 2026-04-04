use std::sync::Arc;

use crate::base::function::WrappedFn;
use crate::base::value::{StaticConcurrent, Value};

pub trait Curry<D, F> {
    fn curry(func: F) -> Self;
}

pub type Curryed1Fn<T1, R> = WrappedFn<T1, R>;
pub type Curryed2Fn<T1, T2, R> = WrappedFn<T1, Curryed1Fn<T2, R>>;
pub type Curryed3Fn<T1, T2, T3, R> = WrappedFn<T1, Curryed2Fn<T2, T3, R>>;
pub type Curryed4Fn<T1, T2, T3, T4, R> = WrappedFn<T1, Curryed3Fn<T2, T3, T4, R>>;
pub type Curryed5Fn<T1, T2, T3, T4, T5, R> = WrappedFn<T1, Curryed4Fn<T2, T3, T4, T5, R>>;
pub type Curryed6Fn<T1, T2, T3, T4, T5, T6, R> = WrappedFn<T1, Curryed5Fn<T2, T3, T4, T5, T6, R>>;
pub type Curryed7Fn<T1, T2, T3, T4, T5, T6, T7, R> =
    WrappedFn<T1, Curryed6Fn<T2, T3, T4, T5, T6, T7, R>>;
pub type Curryed8Fn<T1, T2, T3, T4, T5, T6, T7, T8, R> =
    WrappedFn<T1, Curryed7Fn<T2, T3, T4, T5, T6, T7, T8, R>>;

macro_rules! impl_curry {
    (@closure $func:ident, [$($captured:ident),*], $T:ident $x:ident) => {
        WrappedFn::from(move |$x: $T| -> R {
            $(let $captured = $captured.clone();)*
            (Arc::clone(&$func))($($captured,)* $x)
        })
    };

    (@closure $func:ident, [$($captured:ident),*], $T:ident $x:ident, $($rest_T:ident $rest_x:ident),+) => {
        WrappedFn::from(move |$x: $T| {
            $(let $captured = $captured.clone();)*
            let $func = Arc::clone(&$func);
            impl_curry!(@closure $func, [$($captured,)* $x], $($rest_T $rest_x),+)
        })
    };

    ($type:ident, $($T:ident $x:ident),+) => {
        impl<$($T,)+ R, F> Curry<($($T,)+ R), F> for $type<$($T,)+ R>
        where
            $($T: Value + 'static,)+
            F: Fn($($T),+) -> R + StaticConcurrent,
        {
            fn curry(func: F) -> Self {
                let func = Arc::new(func);
                impl_curry!(@closure func, [], $($T $x),+)
            }
        }
    };
}

impl<T1, R, F> Curry<(T1, R), F> for Curryed1Fn<T1, R>
where
    T1: Value + 'static,
    F: Fn(T1) -> R + StaticConcurrent,
{
    fn curry(func: F) -> Self {
        WrappedFn::from(func)
    }
}

impl_curry!(Curryed2Fn, T1 x1, T2 x2);
impl_curry!(Curryed3Fn, T1 x1, T2 x2, T3 x3);
impl_curry!(Curryed4Fn, T1 x1, T2 x2, T3 x3, T4 x4);
impl_curry!(Curryed5Fn, T1 x1, T2 x2, T3 x3, T4 x4, T5 x5);
impl_curry!(Curryed6Fn, T1 x1, T2 x2, T3 x3, T4 x4, T5 x5, T6 x6);
impl_curry!(Curryed7Fn, T1 x1, T2 x2, T3 x3, T4 x4, T5 x5, T6 x6, T7 x7);
impl_curry!(Curryed8Fn, T1 x1, T2 x2, T3 x3, T4 x4, T5 x5, T6 x6, T7 x7, T8 x8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curryed4fn() {
        let f = WrappedFn::curry(|a, b, c, d| a + b * c - d);
        assert_eq!(f(1)(2)(3)(4), 3);
    }

    #[test]
    fn test_curryed4fn_clone_independence() {
        let f = WrappedFn::curry(|a, b, c, d| a * 1000 + b * 100 + c * 10 + d);
        let f2 = f.clone();
        assert_eq!(f(1)(2)(3)(4), 1234);
        assert_eq!(f2(5)(6)(7)(8), 5678);
    }

    #[test]
    fn test_nested_curryedfn() {
        let f = WrappedFn::curry(|a: i32, b: i32| {
            WrappedFn::curry(move |c: i32, d: i32| a + b + c + d)
        });
        assert_eq!(f(1)(2)(3)(4), 10);
    }
}
