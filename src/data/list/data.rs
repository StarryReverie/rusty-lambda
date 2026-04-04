use std::sync::Arc;

use crate::base::hkt::TypeConstructor1;
use crate::base::value::{Concurrent, SimpleValue, Value};
use crate::data::maybe::Maybe;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ListNode<T> {
    head: T,
    tail: List<T>,
}

impl<T> ListNode<T> {
    fn new(head: T, tail: List<T>) -> Self {
        Self { head, tail }
    }

    pub fn head(&self) -> &T {
        &self.head
    }

    pub fn tail(&self) -> &List<T> {
        &self.tail
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum List<T> {
    #[default]
    Nil,
    Cons(Arc<ListNode<T>>),
}

impl<T> List<T> {
    pub fn empty() -> Self {
        Self::Nil
    }

    pub fn cons(head: T, tail: List<T>) -> Self {
        Self::Cons(Arc::new(ListNode::new(head, tail)))
    }

    pub fn singleton(head: T) -> Self {
        Self::cons(head, Self::empty())
    }

    pub fn head(&self) -> Maybe<&T> {
        match self {
            Self::Cons(node) => Maybe::Just(node.head()),
            Self::Nil => Maybe::Nothing,
        }
    }

    pub fn tail(&self) -> Maybe<&List<T>> {
        match self {
            Self::Cons(node) => Maybe::Just(node.tail()),
            Self::Nil => Maybe::Nothing,
        }
    }
}

impl<T> List<T>
where
    T: Clone,
{
    pub fn append(self, other: List<T>) -> List<T> {
        match self {
            Self::Cons(mut node) => {
                let _ = Arc::make_mut(&mut node);
                let ListNode { head, tail } = Arc::into_inner(node).unwrap();
                List::Cons(Arc::new(ListNode::new(head, tail.append(other))))
            }
            Self::Nil => other,
        }
    }

    pub fn decompose(self) -> Maybe<(T, List<T>)> {
        match self {
            Self::Cons(mut node) => {
                let _ = Arc::make_mut(&mut node);
                let ListNode { head, tail } = Arc::into_inner(node).unwrap();
                Maybe::Just((head, tail))
            }
            Self::Nil => Maybe::Nothing,
        }
    }
}

impl<T> SimpleValue for List<T> where T: Value {}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        fn foldr_recursive<U>(mut iter: impl Iterator<Item = U>) -> List<U> {
            match iter.next() {
                Some(x) => List::cons(x, foldr_recursive(iter)),
                None => List::empty(),
            }
        }
        foldr_recursive(iter.into_iter())
    }
}

impl<I, T> From<I> for List<T>
where
    I: IntoIterator<Item = T>,
{
    fn from(value: I) -> Self {
        Self::from_iter(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ListInstance;

impl TypeConstructor1 for ListInstance {
    type Type<A1>
        = List<A1>
    where
        A1: Concurrent;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_cons() {
        let xs = List::cons(1, List::cons(2, List::cons(3, List::empty())));
        let mut cur = xs.clone();
        for x in [1, 2, 3] {
            assert_eq!(cur.head(), Maybe::Just(&x));
            cur = match cur.tail() {
                Maybe::Just(tail) => tail.clone(),
                _ => unreachable!(),
            };
        }
    }

    #[test]
    fn test_list_from_vec() {
        let xs = List::from(vec![1, 2, 3]);
        let (x1, xs) = xs.decompose().option().unwrap();
        assert_eq!(x1, 1);
        let (x2, xs) = xs.decompose().option().unwrap();
        assert_eq!(x2, 2);
        let (x3, xs) = xs.decompose().option().unwrap();
        assert_eq!(x3, 3);
        assert!(xs.decompose().option().is_none());
    }
}
