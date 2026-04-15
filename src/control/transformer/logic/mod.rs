mod core_instance;
mod lifting_instance;
mod transformer;
mod typeclass;
mod wrapper;

pub use transformer::{LogicT, LogicTInstance, LogicTStep, StackedLogicTInstance};
pub use typeclass::MonadLogic;
pub use wrapper::{Logic, LogicInstance};
