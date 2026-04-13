use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

use crate::base::value::StaticConcurrent;

#[derive(Debug)]
pub struct Thunk<T>(Arc<ThunkInner<T>>);

#[derive(Debug)]
enum ThunkInner<T> {
    Evaluated(T),
    Deferred(LazyLock<Thunk<T>, Box<dyn FnOnce() -> Thunk<T> + Send + Sync + 'static>>),
}

impl<T> Thunk<T> {
    pub fn immediate(value: T) -> Self {
        Self(Arc::new(ThunkInner::Evaluated(value)))
    }

    pub fn lazy<F>(eval: F) -> Self
    where
        F: FnOnce() -> T + StaticConcurrent,
    {
        Self::lazy_monadic(move || Self::immediate(eval()))
    }

    pub fn lazy_monadic<F>(eval: F) -> Self
    where
        F: FnOnce() -> Thunk<T> + StaticConcurrent,
    {
        let inner = ThunkInner::Deferred(LazyLock::new(Box::new(eval)));
        Self(Arc::new(inner))
    }

    pub fn force(&self) -> &T {
        match self.0.as_ref() {
            ThunkInner::Evaluated(value) => value,
            ThunkInner::Deferred(eval) => eval.force(),
        }
    }

    pub fn extract(&self) -> T
    where
        T: Clone,
    {
        self.force().clone()
    }

    pub fn map<U, F>(thunk: Self, func: F) -> Thunk<U>
    where
        T: StaticConcurrent,
        F: FnOnce(&T) -> U + StaticConcurrent,
    {
        Thunk::lazy(move || func(thunk.force()))
    }

    pub fn with<U, F>(thunk: Self, func: F) -> Thunk<U>
    where
        T: StaticConcurrent,
        F: FnOnce(&T) -> Thunk<U> + StaticConcurrent,
    {
        Thunk::lazy_monadic(move || func(thunk.force()))
    }
}

impl<T: Clone> Thunk<T> {}

impl<T> Clone for Thunk<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: PartialEq> PartialEq for Thunk<T> {
    fn eq(&self, other: &Self) -> bool {
        self.force() == other.force()
    }
}

impl<T> Eq for Thunk<T> where T: Eq {}

impl<T: PartialOrd> PartialOrd for Thunk<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.force().partial_cmp(other.force())
    }
}

impl<T: Ord> Ord for Thunk<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.force().cmp(other.force())
    }
}

impl<T: Hash> Hash for Thunk<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.force().hash(state);
    }
}

impl<T: Default + StaticConcurrent> Default for Thunk<T> {
    fn default() -> Self {
        Self::lazy(Default::default)
    }
}

impl<T> From<T> for Thunk<T> {
    fn from(value: T) -> Self {
        Self::immediate(value)
    }
}

impl<T> Deref for Thunk<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.force()
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
