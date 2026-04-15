mod core_instance;
mod lifting_instance;
mod transformer;
mod typeclass;

pub use transformer::{LogicT, LogicTInstance, LogicTStep, StackedLogicTInstance};
pub use typeclass::MonadLogic;
