use crate::base::value::Value;

pub trait ContextConstructor {
    type Type<A>: Value
    where
        A: Value;
}
