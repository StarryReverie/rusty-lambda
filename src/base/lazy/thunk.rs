use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::{Arc, LazyLock};

use crate::base::value::StaticConcurrent;

#[derive(Debug)]
pub struct Thunk<T>(Arc<LazyLock<T, Box<dyn FnOnce() -> T + Send + Sync + 'static>>>);

impl<T> Thunk<T> {
    pub fn immediate(value: T) -> Self
    where
        T: StaticConcurrent,
    {
        Self::lazy(move || value)
    }

    pub fn lazy<F>(eval: F) -> Self
    where
        F: FnOnce() -> T + StaticConcurrent,
    {
        Self(Arc::new(LazyLock::new(Box::new(eval))))
    }

    pub fn force(&self) -> &T {
        self.0.as_ref()
    }
}

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
        self.0.hash(state);
    }
}

impl<T: Default + StaticConcurrent> Default for Thunk<T> {
    fn default() -> Self {
        Self::lazy(Default::default)
    }
}

impl<T, F: FnOnce() -> T + StaticConcurrent> From<F> for Thunk<T> {
    fn from(eval: F) -> Self {
        Self::lazy(eval)
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
