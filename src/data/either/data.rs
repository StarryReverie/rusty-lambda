use std::marker::PhantomData;

use crate::base::value::{SimpleValue, Value};
use crate::control::context::ContextConstructor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Either<E, A> {
    Left(E),
    Right(A),
}

impl<E, A> Either<E, A> {
    pub fn result(self) -> Result<A, E> {
        match self {
            Either::Left(err) => Err(err),
            Either::Right(ok) => Ok(ok),
        }
    }
}

impl<E, A> SimpleValue for Either<E, A>
where
    E: Value,
    A: Value,
{
}

impl<E, A> From<Result<A, E>> for Either<E, A> {
    fn from(value: Result<A, E>) -> Self {
        match value {
            Ok(ok) => Self::Right(ok),
            Err(err) => Self::Left(err),
        }
    }
}

impl<E, A> From<Either<E, A>> for Result<A, E> {
    fn from(value: Either<E, A>) -> Self {
        value.result()
    }
}

impl<E, A> Default for Either<E, A>
where
    E: Default,
{
    fn default() -> Self {
        Self::Left(E::default())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EitherInstance<E>(PhantomData<E>);

impl<E> EitherInstance<E> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<E> Default for EitherInstance<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E> ContextConstructor for EitherInstance<E>
where
    E: Value,
{
    type Type<A>
        = Either<E, A>
    where
        A: Value;
}
