use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::except::{ExceptT, StackedExceptTInstance};
use crate::data::either::Either;

pub type Except<E, A> = ExceptT<E, IdentityInstance, A>;
pub type ExceptInstance<E> = StackedExceptTInstance<E, IdentityInstance>;

impl<E, A> Except<E, A>
where
    E: Value,
    A: Value,
{
    pub fn run(except: Self) -> Either<E, A> {
        Identity::run(ExceptT::run_tr(except))
    }
}
