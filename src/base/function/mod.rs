mod currying;
mod wrapper;

pub use currying::{
    Curry, Curryed1Fn, Curryed2Fn, Curryed3Fn, Curryed4Fn, Curryed5Fn, Curryed6Fn, Curryed7Fn,
    Curryed8Fn,
};
pub use wrapper::{ConcurrentFn, WrappedFn};
