use crate::base::function::{WrappedFn, id};
use crate::base::value::Value;
use crate::control::context::alternative::{Alternative, AlternativeExt};
use crate::control::context::monad::{Monad, MonadExt};

pub use crate::control::context::applicative::lift_a2 as lift_m2;
pub use crate::control::context::applicative::lift_a3 as lift_m3;
pub use crate::control::structure::functor::fmap as lift_m;

pub fn join<MMA, MA, A>(x: MMA) -> MA
where
    MMA: MonadExt<Wrapped = MA> + Value,
    MA: MonadExt<Wrapped = A, Instance = MMA::Instance> + Value,
    A: Value,
    MMA::Instance: Monad<Type<MA> = MMA>,
    MA::Instance: Monad<Type<A> = MA>,
{
    MMA::Instance::bind::<MA, A, WrappedFn<MA, MA>>(x, id())
}

pub fn when<MU>(cond: bool, if_true: MU) -> MU
where
    MU: MonadExt<Wrapped = ()> + Value,
{
    if cond { if_true } else { MU::Instance::ret(()) }
}

pub fn unless<MU>(cond: bool, if_false: MU) -> MU
where
    MU: MonadExt<Wrapped = ()> + Value,
{
    when(!cond, if_false)
}

pub fn guard<MU>(cond: bool) -> MU
where
    MU: MonadExt<Wrapped = ()> + AlternativeExt<Wrapped = ()> + Value,
{
    if cond {
        <MU as MonadExt>::Instance::ret(())
    } else {
        <MU as AlternativeExt>::Instance::fallback::<()>()
    }
}
