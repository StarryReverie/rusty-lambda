use crate::base::value::StaticConcurrent;

pub trait Provider: StaticConcurrent {
    type Capability: Capability;

    fn upcast(self: Box<Self>) -> Box<<Self::Capability as Capability>::Provider>;
}

pub trait Capability: StaticConcurrent {
    type Provider: Provider<Capability = Self> + ?Sized;
}

pub trait Loader<C>: StaticConcurrent
where
    C: Capability,
{
    fn load(&mut self) -> &mut C::Provider;
}
