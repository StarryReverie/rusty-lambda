use std::marker::PhantomData;
use std::sync::Arc;

use crate::base::function::ConcurrentFn;
use crate::base::value::{StaticConcurrent, Value};
use crate::system::io::description::IO;

pub trait IOAction<C>: StaticConcurrent
where
    C: StaticConcurrent,
{
    type Output: Value;

    fn step(&self, context: &mut C) -> IOExecution<C, Self::Output>;

    fn run(&self, context: &mut C) -> Self::Output {
        match self.step(context) {
            IOExecution::Finished(res) => res,
            IOExecution::Deferred(io) => IO::run(io, context),
        }
    }
}

pub enum IOExecution<C, A> {
    Finished(A),
    Deferred(IO<C, A>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IOPureAction<C, A> {
    value: A,
    _marker: PhantomData<C>,
}

impl<C, A> IOPureAction<C, A> {
    pub fn new(value: A) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn boxed(value: A) -> IO<C, A>
    where
        C: StaticConcurrent,
        A: Value,
    {
        IO::new(Arc::new(Self::new(value)))
    }
}

impl<C, A> IOAction<C> for IOPureAction<C, A>
where
    C: StaticConcurrent,
    A: Value,
{
    type Output = A;

    fn step(&self, _context: &mut C) -> IOExecution<C, Self::Output> {
        IOExecution::Finished(self.value.clone())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IODeferredAction<C, F> {
    deferred: F,
    _marker: PhantomData<C>,
}

impl<C, F> IODeferredAction<C, F> {
    pub fn new(deferred: F) -> Self {
        Self {
            deferred,
            _marker: PhantomData,
        }
    }

    pub fn boxed<A>(deferred: F) -> IO<C, A>
    where
        C: StaticConcurrent,
        A: Value,
        F: Fn() -> IO<C, A> + StaticConcurrent,
    {
        IO::new(Arc::new(Self::new(deferred)))
    }
}

impl<C, A, F> IOAction<C> for IODeferredAction<C, F>
where
    C: StaticConcurrent,
    A: Value,
    F: Fn() -> IO<C, A> + StaticConcurrent,
{
    type Output = A;

    fn step(&self, _context: &mut C) -> IOExecution<C, Self::Output> {
        IOExecution::Deferred((self.deferred)())
    }
}

#[derive(Clone)]
pub struct IOMapAction<C, A, G> {
    prev: IO<C, A>,
    mapper: G,
}

impl<C, A, G> IOMapAction<C, A, G> {
    pub fn new(prev: IO<C, A>, mapper: G) -> Self {
        Self { prev, mapper }
    }

    pub fn boxed<B>(prev: IO<C, A>, mapper: G) -> IO<C, B>
    where
        C: StaticConcurrent,
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        IO::new(Arc::new(Self::new(prev, mapper)))
    }
}

impl<C, A, B, G> IOAction<C> for IOMapAction<C, A, G>
where
    C: StaticConcurrent,
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
{
    type Output = B;

    fn step(&self, context: &mut C) -> IOExecution<C, Self::Output> {
        let res = IO::run(&self.prev, context);
        IOExecution::Finished((self.mapper).view().call(res))
    }
}

#[derive(Clone)]
pub struct IOBindAction<C, A, G> {
    prev: IO<C, A>,
    binder: G,
}

impl<C, A, G> IOBindAction<C, A, G> {
    pub fn new(prev: IO<C, A>, binder: G) -> Self {
        Self { prev, binder }
    }

    pub fn boxed<B>(io: IO<C, A>, binder: G) -> IO<C, B>
    where
        C: StaticConcurrent,
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = IO<C, B>>>,
    {
        IO::new(Arc::new(Self::new(io, binder)))
    }
}

impl<C, A, B, G> IOAction<C> for IOBindAction<C, A, G>
where
    C: StaticConcurrent,
    A: Value,
    B: Value,
    G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = IO<C, B>>>,
{
    type Output = B;

    fn step(&self, context: &mut C) -> IOExecution<C, Self::Output> {
        let res = IO::run(&self.prev, context);
        IOExecution::Deferred(self.binder.view().call(res))
    }
}
