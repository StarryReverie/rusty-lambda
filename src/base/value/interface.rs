pub trait Concurrent: Send + Sync {}

impl<T> Concurrent for T where T: Send + Sync {}

pub trait StaticConcurrent: Concurrent + 'static {}

impl<T> StaticConcurrent for T where T: Concurrent + 'static {}

pub trait Value: StaticConcurrent + Clone {}

impl<T> Value for T where T: StaticConcurrent + Clone {}
