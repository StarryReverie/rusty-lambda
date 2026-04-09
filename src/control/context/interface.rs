use crate::base::value::{StaticConcurrent, Value};

pub trait ContextConstructor: Clone + StaticConcurrent {
    type Type<A>: Value
    where
        A: Value;
}
