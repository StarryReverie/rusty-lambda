use std::ops::Mul;

use crate::base::value::Value;

pub trait Multiplicative: Value + Mul<Output = Self> {
    fn one() -> Self;
}

macro_rules! impl_multiplicative {
    ($t:ty, $one:expr) => {
        impl Multiplicative for $t {
            fn one() -> Self {
                $one
            }
        }
    };
}

impl_multiplicative!(i8, 1);
impl_multiplicative!(i16, 1);
impl_multiplicative!(i32, 1);
impl_multiplicative!(i64, 1);
impl_multiplicative!(i128, 1);
impl_multiplicative!(isize, 1);
impl_multiplicative!(u8, 1);
impl_multiplicative!(u16, 1);
impl_multiplicative!(u32, 1);
impl_multiplicative!(u64, 1);
impl_multiplicative!(u128, 1);
impl_multiplicative!(usize, 1);
impl_multiplicative!(f32, 1.0);
impl_multiplicative!(f64, 1.0);
