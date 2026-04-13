use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

use crate::base::computation::{Computation, LazyReturn};
use crate::base::value::StaticConcurrent;

#[derive(Debug)]
pub struct Thunk<T>(Arc<LazyLock<T, LazyReturn<T>>>);

impl<T> Thunk<T>
where
    T: StaticConcurrent,
{
    pub fn new(computation: Computation<T>) -> Self {
        Self(Arc::new(LazyLock::new(computation.lazy_eval())))
    }

    pub fn immediate(value: T) -> Self {
        Self::new(Computation::immediate(value))
    }

    pub fn lazy<F>(eval: F) -> Self
    where
        F: FnOnce() -> T + StaticConcurrent,
    {
        Self::new(Computation::lazy(eval))
    }

    pub fn map<U, F>(thunk: Self, func: F) -> Thunk<U>
    where
        U: StaticConcurrent,
        F: FnOnce(&T) -> U + StaticConcurrent,
    {
        Thunk::lazy(move || func(Thunk::force(&thunk)))
    }

    pub fn bind<U, F>(thunk: Self, func: F) -> Thunk<U>
    where
        U: StaticConcurrent,
        F: FnOnce(&T) -> Computation<U> + StaticConcurrent,
    {
        Thunk::new(Computation::monadic(move || func(Thunk::force(&thunk))))
    }
}

impl<T> Thunk<T> {
    pub fn force(thunk: &Self) -> &T {
        LazyLock::force(&thunk.0)
    }

    pub fn extract(thunk: &Self) -> T
    where
        T: Clone,
    {
        Thunk::force(thunk).clone()
    }
}

impl<T> Clone for Thunk<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> PartialEq for Thunk<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        Thunk::force(self) == Thunk::force(other)
    }
}

impl<T> Eq for Thunk<T> where T: Eq {}

impl<T> PartialOrd for Thunk<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Thunk::force(self).partial_cmp(Thunk::force(other))
    }
}

impl<T> Ord for Thunk<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Thunk::force(self).cmp(Thunk::force(other))
    }
}

impl<T> Hash for Thunk<T>
where
    T: Hash,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        Thunk::force(self).hash(state);
    }
}

impl<T> Default for Thunk<T>
where
    T: Default + StaticConcurrent,
{
    fn default() -> Self {
        Self::lazy(Default::default)
    }
}

impl<T> From<T> for Thunk<T>
where
    T: StaticConcurrent,
{
    fn from(value: T) -> Self {
        Self::immediate(value)
    }
}

impl<T> Deref for Thunk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Thunk::force(self)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[test]
    fn test_thunk_immediate_eval() {
        let x = Thunk::immediate(42);
        assert_eq!(*x, 42);
    }

    #[test]
    fn test_thunk_lazy_eval() {
        let evaluated = Arc::new(Mutex::new(false));
        let x = {
            let evaluated = evaluated.clone();
            Thunk::lazy(move || {
                *evaluated.lock().unwrap() = true;
                42
            })
        };
        assert_eq!(*evaluated.lock().unwrap(), false);
        assert_eq!(*x, 42);
        assert_eq!(*evaluated.lock().unwrap(), true);
    }
}
