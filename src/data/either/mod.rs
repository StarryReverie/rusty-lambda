mod applicative;
mod data;
mod foldable;
mod functor;
mod monad;
mod semigroup;
mod traversable;
mod util;

pub use data::{Either, EitherInstance};
pub use util::{
    either, from_left, from_right, is_left, is_right, lefts, partition_eithers, rights,
};
