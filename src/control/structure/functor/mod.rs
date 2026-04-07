pub mod identity;

mod derive;
mod typeclass;
mod util;

pub use typeclass::{Functor, FunctorExt, LhsFunctorExt};
pub use util::{fmap, void};
