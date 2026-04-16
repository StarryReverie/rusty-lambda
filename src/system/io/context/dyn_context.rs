use std::any::{Any, TypeId, type_name};
use std::collections::HashMap;

use crate::system::io::context::{Capability, Loader, Provider};

#[derive(Default)]
pub struct DynContext(HashMap<TypeId, Box<dyn Any + Send + Sync + 'static>>);

impl DynContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with<P>(mut self, provider: Box<P>) -> Self
    where
        P: Provider + ?Sized,
    {
        self.register(provider);
        self
    }

    pub fn register<P>(&mut self, provider: Box<P>) -> &mut Self
    where
        P: Provider + ?Sized,
    {
        let key = TypeId::of::<P::Capability>();
        self.0.insert(key, Box::new(provider.upcast()));
        self
    }
}

impl<C> Loader<C> for DynContext
where
    C: Capability,
{
    fn load(&mut self) -> &mut <C as Capability>::Provider {
        let key = TypeId::of::<C>();
        let erased = (self.0)
            .get_mut(&key)
            .unwrap_or_else(|| panic!("capability {} not registered", type_name::<C>()))
            .as_mut();
        let concrete = erased
            .downcast_mut::<Box<<C as Capability>::Provider>>()
            .unwrap_or_else(|| {
                panic!(
                    "could not downcast to a concrete provider {}",
                    type_name::<<C as Capability>::Provider>()
                )
            })
            .as_mut();
        concrete
    }
}
