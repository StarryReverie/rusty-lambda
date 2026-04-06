use crate::base::numeric::{Additive, Multiplicative, Subtractive};
use crate::base::value::Value;

pub trait Num: Value + Additive + Subtractive + Multiplicative {
    fn from_integer(num: i64) -> Self;
}

macro_rules! impl_num {
    ($t:ty) => {
        impl Num for $t {
            fn from_integer(num: i64) -> Self {
                num as $t
            }
        }
    };
}

impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(i128);
impl_num!(isize);
impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(u128);
impl_num!(usize);
impl_num!(f32);
impl_num!(f64);

pub trait SignedNum: Num {
    fn abs(self) -> Self;

    fn signum(self) -> Self;
}

macro_rules! impl_signed_num_int {
    ($t:ty) => {
        impl SignedNum for $t {
            fn abs(self) -> Self {
                self.wrapping_abs()
            }

            fn signum(self) -> Self {
                self.signum()
            }
        }
    };
}

macro_rules! impl_signed_num_float {
    ($t:ty) => {
        impl SignedNum for $t {
            fn abs(self) -> Self {
                self.abs()
            }

            fn signum(self) -> Self {
                self.signum()
            }
        }
    };
}

impl_signed_num_int!(i8);
impl_signed_num_int!(i16);
impl_signed_num_int!(i32);
impl_signed_num_int!(i64);
impl_signed_num_int!(i128);
impl_signed_num_int!(isize);
impl_signed_num_float!(f32);
impl_signed_num_float!(f64);
