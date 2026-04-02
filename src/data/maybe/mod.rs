mod applicative;
mod data;
mod functor;
mod monad;
mod util;

pub use data::{Maybe, MaybeInstance};
pub use util::{from_maybe, is_just, is_nothing, maybe};
