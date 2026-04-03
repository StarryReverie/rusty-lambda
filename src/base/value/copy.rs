use crate::base::value::{Concurrent, Value};

impl<T> Value for T
where
    T: Primitive + Concurrent,
{
    type Unwrapped = T;

    type View<'a>
        = T
    where
        Self: 'a;

    fn make<U>(unwrapped: U) -> Self
    where
        U: Into<Self::Unwrapped>,
        Self::Unwrapped: Sized,
    {
        unwrapped.into()
    }

    fn view(&self) -> Self::View<'_> {
        *self
    }
}

trait Primitive: Copy {}

impl Primitive for () {}

impl Primitive for bool {}

impl Primitive for char {}

impl Primitive for i8 {}
impl Primitive for i16 {}
impl Primitive for i32 {}
impl Primitive for i64 {}
impl Primitive for i128 {}
impl Primitive for isize {}

impl Primitive for u8 {}
impl Primitive for u16 {}
impl Primitive for u32 {}
impl Primitive for u64 {}
impl Primitive for u128 {}
impl Primitive for usize {}

impl<T> Primitive for &T {}

impl Primitive for &str {}

impl<T> Primitive for &[T] {}

impl<T, const N: usize> Primitive for [T; N] where T: Copy {}

impl<T> Primitive for *const T {}

impl<T1> Primitive for (T1,) where T1: Copy {}

impl<T1, T2> Primitive for (T1, T2) where (T1, T2): Copy {}

impl<T1, T2, T3> Primitive for (T1, T2, T3) where (T1, T2, T3): Copy {}

impl<T1, T2, T3, T4> Primitive for (T1, T2, T3, T4) where (T1, T2, T3, T4): Copy {}

impl<T1, T2, T3, T4, T5> Primitive for (T1, T2, T3, T4, T5) where (T1, T2, T3, T4, T5): Copy {}

impl<T1, T2, T3, T4, T5, T6> Primitive for (T1, T2, T3, T4, T5, T6) where
    (T1, T2, T3, T4, T5, T6): Copy
{
}

impl<T1, T2, T3, T4, T5, T6, T7> Primitive for (T1, T2, T3, T4, T5, T6, T7) where
    (T1, T2, T3, T4, T5, T6, T7): Copy
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> Primitive for (T1, T2, T3, T4, T5, T6, T7, T8) where
    (T1, T2, T3, T4, T5, T6, T7, T8): Copy
{
}

impl<T1, R> Primitive for fn(T1) -> R {}

impl<T1, T2, R> Primitive for fn(T1, T2) -> R {}

impl<T1, T2, T3, R> Primitive for fn(T1, T2, T3) -> R {}

impl<T1, T2, T3, T4, R> Primitive for fn(T1, T2, T3, T4) -> R {}

impl<T1, T2, T3, T4, T5, R> Primitive for fn(T1, T2, T3, T4, T5) -> R {}

impl<T1, T2, T3, T4, T5, T6, R> Primitive for fn(T1, T2, T3, T4, T5, T6) -> R {}

impl<T1, T2, T3, T4, T5, T6, T7, R> Primitive for fn(T1, T2, T3, T4, T5, T6, T7) -> R {}

impl<T1, T2, T3, T4, T5, T6, T7, T8, R> Primitive for fn(T1, T2, T3, T4, T5, T6, T7, T8) -> R {}
