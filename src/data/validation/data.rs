use std::marker::PhantomData;

use crate::base::value::{SimpleValue, StaticConcurrent, Value};
use crate::control::context::ContextConstructor;
use crate::control::structure::monoid::Monoid;
use crate::data::either::Either;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Validation<E, A> {
    Success(A),
    Failure(E),
}

impl<E, A> Validation<E, A> {
    pub fn either(self) -> Either<E, A> {
        match self {
            Validation::Success(x) => Either::Right(x),
            Validation::Failure(e) => Either::Left(e),
        }
    }

    pub fn result(self) -> Result<A, E> {
        match self {
            Validation::Success(x) => Ok(x),
            Validation::Failure(e) => Err(e),
        }
    }
}

impl<E, A> SimpleValue for Validation<E, A>
where
    E: Value,
    A: Value,
{
}

impl<E, A> From<Result<A, E>> for Validation<E, A> {
    fn from(value: Result<A, E>) -> Self {
        match value {
            Ok(ok) => Self::Success(ok),
            Err(err) => Self::Failure(err),
        }
    }
}

impl<E, A> From<Validation<E, A>> for Result<A, E> {
    fn from(value: Validation<E, A>) -> Self {
        value.result()
    }
}

impl<E, A> From<Either<E, A>> for Validation<E, A> {
    fn from(value: Either<E, A>) -> Self {
        match value {
            Either::Right(x) => Self::Success(x),
            Either::Left(e) => Self::Failure(e),
        }
    }
}

impl<E, A> From<Validation<E, A>> for Either<E, A> {
    fn from(value: Validation<E, A>) -> Self {
        value.either()
    }
}

impl<E, A> Default for Validation<E, A>
where
    E: Monoid,
{
    fn default() -> Self {
        Validation::Failure(E::empty())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ValidationInstance<E>(PhantomData<E>);

impl<E> ContextConstructor for ValidationInstance<E>
where
    E: StaticConcurrent,
{
    type Type<A>
        = Validation<E, A>
    where
        A: StaticConcurrent;
}
