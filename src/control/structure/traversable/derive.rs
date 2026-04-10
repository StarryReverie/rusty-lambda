#[macro_export]
macro_rules! derive_traversable_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::structure::traversable::Traversable for $instance {
            fn traverse<F, A, B, FB, G>(map: G, container: Self::Type<A>) -> F::Type<Self::Type<B>>
            where
                F: $crate::control::context::applicative::Applicative<Type<B> = FB>,
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                FB: $crate::control::context::applicative::ApplicativeExt<Wrapped = B, Instance = F>
                    + $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = FB>,
                    >,
            {
                F::fmap(
                    &(|x| $wrapper(x)),
                    $crate::base::function::ConcurrentFn::call(&map.view(), container.0),
                )
            }
        }
    };
}

#[macro_export]
macro_rules! derive_traversable_for_nested_traversable {
    ($instance:ty, $wrapper:ident, $inner_instance:ty) => {
        impl $crate::control::structure::traversable::Traversable for $instance {
            fn traverse<F, A, B, FB, G>(
                map: G,
                container: Self::Type<A>,
            ) -> F::Type<Self::Type<B>>
            where
                F: $crate::control::context::applicative::Applicative<Type<B> = FB>,
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                FB: $crate::control::context::applicative::ApplicativeExt<Wrapped = B, Instance = F>
                    + $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                    View<'a>: $crate::base::function::ConcurrentFn<A, Output = FB>,
                >,
            {
                F::fmap(
                    &(|x| $wrapper(x)),
                    <$inner_instance as $crate::control::structure::traversable::Traversable>::traverse(
                        map, container.0,
                    ),
                )
            }
        }
    };
}
