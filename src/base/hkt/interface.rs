use crate::base::value::StaticConcurrent;

pub trait TypeConstructor1 {
    type Type<A1>: StaticConcurrent
    where
        A1: StaticConcurrent;
}

pub trait TypeConstructor2 {
    type Type<A1, A2>: StaticConcurrent
    where
        A1: StaticConcurrent,
        A2: StaticConcurrent;
}

pub trait TypeConstructor3 {
    type Type<A1, A2, A3>: StaticConcurrent
    where
        A1: StaticConcurrent,
        A2: StaticConcurrent,
        A3: StaticConcurrent;
}

pub trait TypeConstructor4 {
    type Type<A1, A2, A3, A4>: StaticConcurrent
    where
        A1: StaticConcurrent,
        A2: StaticConcurrent,
        A3: StaticConcurrent,
        A4: StaticConcurrent;
}
