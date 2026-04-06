use crate::base::value::{SimpleValue, StaticConcurrent};

impl<T> SimpleValue for T where T: Primitive + StaticConcurrent {}

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

impl Primitive for f32 {}
impl Primitive for f64 {}

impl<T> Primitive for &T {}

impl Primitive for &str {}

impl<T> Primitive for &[T] {}

impl<T, const N: usize> Primitive for [T; N] where T: Copy {}

impl<T> Primitive for *const T {}

impl<T1, R> Primitive for fn(T1) -> R {}

impl<T1, T2, R> Primitive for fn(T1, T2) -> R {}

impl<T1, T2, T3, R> Primitive for fn(T1, T2, T3) -> R {}

impl<T1, T2, T3, T4, R> Primitive for fn(T1, T2, T3, T4) -> R {}

impl<T1, T2, T3, T4, T5, R> Primitive for fn(T1, T2, T3, T4, T5) -> R {}

impl<T1, T2, T3, T4, T5, T6, R> Primitive for fn(T1, T2, T3, T4, T5, T6) -> R {}

impl<T1, T2, T3, T4, T5, T6, T7, R> Primitive for fn(T1, T2, T3, T4, T5, T6, T7) -> R {}

impl<T1, T2, T3, T4, T5, T6, T7, T8, R> Primitive for fn(T1, T2, T3, T4, T5, T6, T7, T8) -> R {}
