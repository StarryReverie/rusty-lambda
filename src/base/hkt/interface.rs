use crate::base::value::Concurrent;

pub trait TypeConstructor1 {
    type Type<A1>: Concurrent
    where
        A1: Concurrent;
}

pub trait TypeConstructor2 {
    type Type<A1, A2>: Concurrent
    where
        A1: Concurrent,
        A2: Concurrent;
}

pub trait TypeConstructor3 {
    type Type<A1, A2, A3>: Concurrent
    where
        A1: Concurrent,
        A2: Concurrent,
        A3: Concurrent;
}

pub trait TypeConstructor4 {
    type Type<A1, A2, A3, A4>: Concurrent
    where
        A1: Concurrent,
        A2: Concurrent,
        A3: Concurrent,
        A4: Concurrent;
}
