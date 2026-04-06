#[macro_export]
macro_rules! derive_functor_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::structure::functor::Functor for $instance {
            fn fmap<A, B, G>(g: G, x: Self::Type<A>) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = B>,
                    >,
            {
                $wrapper($crate::base::function::ConcurrentFn::call(&g.view(), x.0))
            }
        }
    };
}
