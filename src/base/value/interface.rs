pub trait Concurrent: Send + Sync {}

impl<T> Concurrent for T where T: Send + Sync + ?Sized {}

pub trait StaticConcurrent: Concurrent + 'static {}

impl<T> StaticConcurrent for T where T: Concurrent + ?Sized + 'static {}

pub trait Value: StaticConcurrent + Clone {
    type View<'a>: Concurrent + 'a
    where
        Self: 'a;

    fn view(&self) -> Self::View<'_>;
}

pub trait SimpleValue: StaticConcurrent + Clone {}

impl<T> Value for T
where
    T: SimpleValue,
{
    type View<'a>
        = &'a Self
    where
        Self: 'a;

    fn view(&self) -> Self::View<'_> {
        self
    }
}

impl<T1> SimpleValue for (T1,) where T1: Value {}

impl<T1, T2> SimpleValue for (T1, T2)
where
    T1: Value,
    T2: Value,
{
}

impl<T1, T2, T3> SimpleValue for (T1, T2, T3)
where
    T1: Value,
    T2: Value,
    T3: Value,
{
}

impl<T1, T2, T3, T4> SimpleValue for (T1, T2, T3, T4)
where
    T1: Value,
    T2: Value,
    T3: Value,
    T4: Value,
{
}

impl<T1, T2, T3, T4, T5> SimpleValue for (T1, T2, T3, T4, T5)
where
    T1: Value,
    T2: Value,
    T3: Value,
    T4: Value,
    T5: Value,
{
}

impl<T1, T2, T3, T4, T5, T6> SimpleValue for (T1, T2, T3, T4, T5, T6)
where
    T1: Value,
    T2: Value,
    T3: Value,
    T4: Value,
    T5: Value,
    T6: Value,
{
}

impl<T1, T2, T3, T4, T5, T6, T7> SimpleValue for (T1, T2, T3, T4, T5, T6, T7)
where
    T1: Value,
    T2: Value,
    T3: Value,
    T4: Value,
    T5: Value,
    T6: Value,
    T7: Value,
{
}

impl<T1, T2, T3, T4, T5, T6, T7, T8> SimpleValue for (T1, T2, T3, T4, T5, T6, T7, T8)
where
    T1: Value,
    T2: Value,
    T3: Value,
    T4: Value,
    T5: Value,
    T6: Value,
    T7: Value,
    T8: Value,
{
}
