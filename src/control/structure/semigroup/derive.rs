#[macro_export]
macro_rules! derive_semigroup_for_wrapper {
    (for<$param:ident>, $wrapper:ty) => {
        impl<$param> $crate::control::structure::semigroup::Semigroup for $wrapper
        where
            $param: $crate::control::structure::semigroup::Semigroup,
        {
            fn associate(self, other: Self) -> Self {
                Self($crate::control::structure::semigroup::Semigroup::associate(
                    self.0, other.0,
                ))
            }
        }
    };
}
