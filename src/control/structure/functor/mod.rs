pub mod identity;

mod derive;
mod typeclass;
mod util;

pub use typeclass::{Functor, FunctorExt};
pub use util::fmap;
