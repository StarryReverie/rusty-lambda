use crate::base::value::StaticConcurrent;

pub trait ContextConstructor {
    type Type<A>: StaticConcurrent
    where
        A: StaticConcurrent;
}
