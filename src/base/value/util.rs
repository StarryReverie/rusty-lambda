use std::sync::Arc;

pub fn arc<T>(unwrapped: T) -> Arc<T> {
    Arc::new(unwrapped)
}
