use std::borrow::Borrow;

use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::logic::{LogicT, StackedLogicTInstance};
use crate::data::list::List;
use crate::data::maybe::Maybe;

pub type Logic<A> = LogicT<IdentityInstance, A>;
pub type LogicInstance = StackedLogicTInstance<IdentityInstance>;

impl<A> Logic<A>
where
    A: Value,
{
    pub fn observe(logic: impl Borrow<Self>) -> Maybe<A> {
        Identity::run(Self::observe_tr(logic))
    }

    pub fn observe_many(n: usize, logic: impl Borrow<Self>) -> List<A> {
        Identity::run(Self::observe_many_tr(n, logic))
    }

    pub fn observe_all(logic: impl Borrow<Self>) -> List<A> {
        Identity::run(Self::observe_all_tr(logic))
    }
}
