#[macro_export]
macro_rules! derive_alternative_for_nested_alternative {
    ($instance:ty, $wrapper:ident, $inner_instance:ty) => {
        impl $crate::control::context::alternative::Alternative for $instance {
            fn fallback<A>() -> Self::Type<A>
            where
                A: $crate::base::value::Value,
            {
                $wrapper(
                    <$inner_instance as $crate::control::context::alternative::Alternative>::fallback(),
                )
            }

            fn alt<A>(one: Self::Type<A>, another: Self::Type<A>) -> Self::Type<A>
            where
                A: $crate::base::value::Value,
            {
                $wrapper(
                    <$inner_instance as $crate::control::context::alternative::Alternative>::alt(
                        one.0, another.0,
                    ),
                )
            }
        }

        impl<T> $crate::control::context::alternative::AlternativeExt for $wrapper<T>
        where
            T: $crate::base::value::Value,
        {
            type Wrapped = T;
            type Instance = $instance;
        }
    };
}
