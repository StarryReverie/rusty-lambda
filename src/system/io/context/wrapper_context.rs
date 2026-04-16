use crate::system::io::context::{Capability, Loader, Provider};

pub struct WrapperContext<P>(Box<P>)
where
    P: Provider + ?Sized;

impl<P> WrapperContext<P>
where
    P: Provider + ?Sized,
{
    pub fn new(provider: Box<P>) -> Self {
        Self(provider)
    }
}

impl<C> Loader<C> for WrapperContext<C::Provider>
where
    C: Capability,
{
    fn load(&mut self) -> &mut C::Provider {
        &mut self.0
    }
}
