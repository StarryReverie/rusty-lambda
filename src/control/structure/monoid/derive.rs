#[macro_export]
macro_rules! derive_monoid_for_wrapper {
    (for<$param:ident>, $wrapper:ty) => {
        impl<$param> $crate::control::structure::monoid::Monoid for $wrapper
        where
            $param: $crate::control::structure::monoid::Monoid,
        {
            fn empty() -> Self {
                Self($param::empty())
            }
        }
    };
}
