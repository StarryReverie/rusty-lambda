use std::sync::Arc;

use crate::base::function::WrappedFn;
use crate::base::value::{StaticConcurrent, Value};

macro_rules! impl_curryed_fn {
    ($name:ident < $($ty:ident),* >, $first:ident, $output:ty) => {
        impl<$($ty),*> $crate::base::function::ConcurrentFn<$first> for $name<$($ty),*>
        // where
        //     $($ty: 'static,)*
        {
            type Output = $output;

            fn call(&self, argument: $first) -> Self::Output {
                self.0.call(argument)
            }
        }

        impl<$($ty),*> $crate::base::value::Value for $name<$($ty),*> {
            type Unwrapped = Self;

            type View<'a>
                = &'a <Self as ::std::ops::Deref>::Target
            where
                Self: 'a;

            fn make<U>(unwrapped: U) -> Self
            where
                U: ::std::convert::Into<Self::Unwrapped>,
                Self::Unwrapped: ::std::marker::Sized,
            {
                unwrapped.into()
            }

            fn view(&self) -> Self::View<'_> {
                &**self
            }
        }

        impl<$($ty),*> ::std::ops::Deref for $name<$($ty),*> {
            type Target = dyn ::std::ops::Fn($first) -> $output + ::std::marker::Send + ::std::marker::Sync;

            fn deref(&self) -> &Self::Target {
                &*self.0
            }
        }

        impl<$($ty),*> ::std::clone::Clone for $name<$($ty),*> {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }
    };
}

pub struct Curryed1Fn<T1, R>(WrappedFn<T1, R>);

impl_curryed_fn!(Curryed1Fn<T1, R>, T1, R);

impl<T1, R, F> From<F> for Curryed1Fn<T1, R>
where
    F: Fn(T1) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        Self(WrappedFn::from(func))
    }
}

pub struct Curryed2Fn<T1, T2, R>(WrappedFn<T1, Curryed1Fn<T2, R>>);

impl_curryed_fn!(Curryed2Fn<T1, T2, R>, T1, Curryed1Fn<T2, R>);

impl<T1, T2, R, F> From<F> for Curryed2Fn<T1, T2, R>
where
    T1: Value + 'static,
    F: Fn(T1, T2) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        let func = Arc::new(func);
        Self(WrappedFn::from(move |x1: T1| -> Curryed1Fn<T2, R> {
            let func = Arc::clone(&func);
            Curryed1Fn(WrappedFn::from(move |x2: T2| -> R {
                let x1 = x1.clone();
                (Arc::clone(&func))(x1, x2)
            }))
        }))
    }
}

pub struct Curryed3Fn<T1, T2, T3, R>(WrappedFn<T1, Curryed2Fn<T2, T3, R>>);

impl_curryed_fn!(Curryed3Fn<T1, T2, T3, R>, T1, Curryed2Fn<T2, T3, R>);

impl<T1, T2, T3, R, F> From<F> for Curryed3Fn<T1, T2, T3, R>
where
    T1: Value + 'static,
    T2: Value + 'static,
    F: Fn(T1, T2, T3) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        let func = Arc::new(func);
        Self(WrappedFn::from(move |x1: T1| -> Curryed2Fn<T2, T3, R> {
            let func = Arc::clone(&func);
            Curryed2Fn(WrappedFn::from(move |x2: T2| -> Curryed1Fn<T3, R> {
                let x1 = x1.clone();
                let func = Arc::clone(&func);
                Curryed1Fn(WrappedFn::from(move |x3: T3| -> R {
                    let (x1, x2) = (x1.clone(), x2.clone());
                    (Arc::clone(&func))(x1, x2, x3)
                }))
            }))
        }))
    }
}

pub struct Curryed4Fn<T1, T2, T3, T4, R>(WrappedFn<T1, Curryed3Fn<T2, T3, T4, R>>);

impl_curryed_fn!(Curryed4Fn<T1, T2, T3, T4, R>, T1, Curryed3Fn<T2, T3, T4, R>);

impl<T1, T2, T3, T4, R, F> From<F> for Curryed4Fn<T1, T2, T3, T4, R>
where
    T1: Value + 'static,
    T2: Value + 'static,
    T3: Value + 'static,
    F: Fn(T1, T2, T3, T4) -> R + StaticConcurrent,
{
    fn from(func: F) -> Self {
        let func = Arc::new(func);
        Self(WrappedFn::from(
            move |x1: T1| -> Curryed3Fn<T2, T3, T4, R> {
                let func = Arc::clone(&func);
                Curryed3Fn(WrappedFn::from(move |x2: T2| -> Curryed2Fn<T3, T4, R> {
                    let x1 = x1.clone();
                    let func = Arc::clone(&func);
                    Curryed2Fn(WrappedFn::from(move |x3: T3| -> Curryed1Fn<T4, R> {
                        let (x1, x2) = (x1.clone(), x2.clone());
                        let func = Arc::clone(&func);
                        Curryed1Fn(WrappedFn::from(move |x4: T4| -> R {
                            let (x1, x2, x3) = (x1.clone(), x2.clone(), x3.clone());
                            (Arc::clone(&func))(x1, x2, x3, x4)
                        }))
                    }))
                }))
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curryed1fn() {
        let f: Curryed1Fn<i32, i32> = Curryed1Fn::from(|x: i32| x * 2);
        assert_eq!(f(3), 6);
    }

    #[test]
    fn test_curryed2fn() {
        let f: Curryed2Fn<i32, i32, i32> = Curryed2Fn::from(|x, y| x + y);
        assert_eq!(f(1)(2), 3);
    }

    #[test]
    fn test_curryed3fn() {
        let f: Curryed3Fn<i32, i32, i32, i32> = Curryed3Fn::from(|x, y, z| x * y + z);
        assert_eq!(f(2)(3)(1), 7);
    }

    #[test]
    fn test_curryed4fn() {
        let f: Curryed4Fn<i32, i32, i32, i32, i32> = Curryed4Fn::from(|a, b, c, d| a + b * c - d);
        assert_eq!(f(1)(2)(3)(4), 3);
    }

    #[test]
    fn test_curryed4fn_clone_independence() {
        let f: Curryed4Fn<i32, i32, i32, i32, i32> =
            Curryed4Fn::from(|a, b, c, d| a * 1000 + b * 100 + c * 10 + d);
        let f2 = f.clone();
        assert_eq!(f(1)(2)(3)(4), 1234);
        assert_eq!(f2(5)(6)(7)(8), 5678);
    }
}
