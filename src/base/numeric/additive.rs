use std::ops::{Add, Sub};

use crate::base::value::Value;

pub trait Additive: Value + Add<Output = Self> {
    fn zero() -> Self;
}

macro_rules! impl_additive {
    ($t:ty, $zero:expr) => {
        impl Additive for $t {
            fn zero() -> Self {
                $zero
            }
        }
    };
}

impl_additive!(i8, 0);
impl_additive!(i16, 0);
impl_additive!(i32, 0);
impl_additive!(i64, 0);
impl_additive!(i128, 0);
impl_additive!(isize, 0);
impl_additive!(u8, 0);
impl_additive!(u16, 0);
impl_additive!(u32, 0);
impl_additive!(u64, 0);
impl_additive!(u128, 0);
impl_additive!(usize, 0);
impl_additive!(f32, 0.0);
impl_additive!(f64, 0.0);

pub trait Subtractive: Value + Additive + Sub<Output = Self> {
    fn negate(self) -> Self;
}

macro_rules! impl_subtractive_wrapping {
    ($t:ty) => {
        impl Subtractive for $t {
            fn negate(self) -> Self {
                self.wrapping_neg()
            }
        }
    };
}

macro_rules! impl_subtractive_neg {
    ($t:ty) => {
        impl Subtractive for $t {
            fn negate(self) -> Self {
                -self
            }
        }
    };
}

impl_subtractive_wrapping!(i8);
impl_subtractive_wrapping!(i16);
impl_subtractive_wrapping!(i32);
impl_subtractive_wrapping!(i64);
impl_subtractive_wrapping!(i128);
impl_subtractive_wrapping!(isize);
impl_subtractive_wrapping!(u8);
impl_subtractive_wrapping!(u16);
impl_subtractive_wrapping!(u32);
impl_subtractive_wrapping!(u64);
impl_subtractive_wrapping!(u128);
impl_subtractive_wrapping!(usize);
impl_subtractive_neg!(f32);
impl_subtractive_neg!(f64);
