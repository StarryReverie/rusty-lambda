use std::borrow::Borrow;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::base::value::{SimpleValue, StaticConcurrent, Value};
use crate::control::context::ContextConstructor;
use crate::system::io::description::{IOAction, IOExecution};

pub struct IO<C, A>(Arc<dyn IOAction<C, Output = A>>);

impl<C, A> IO<C, A> {
    pub fn new(action: Arc<dyn IOAction<C, Output = A>>) -> Self {
        Self(action)
    }

    pub fn run(io: impl Borrow<Self>, context: &mut C) -> A
    where
        C: StaticConcurrent,
        A: Value,
    {
        match io.borrow().0.step(context) {
            IOExecution::Finished(res) => res,
            IOExecution::Deferred(mut current) => loop {
                match current.0.step(context) {
                    IOExecution::Finished(res) => break res,
                    IOExecution::Deferred(io) => current = io,
                }
            },
        }
    }
}

impl<C, A> Clone for IO<C, A> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<C, A> SimpleValue for IO<C, A>
where
    C: StaticConcurrent,
    A: StaticConcurrent,
{
}

#[derive(Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IOInstance<C>(PhantomData<C>);

impl<C> Clone for IOInstance<C> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<C> ContextConstructor for IOInstance<C>
where
    C: StaticConcurrent,
{
    type Type<A>
        = IO<C, A>
    where
        A: Value;
}
