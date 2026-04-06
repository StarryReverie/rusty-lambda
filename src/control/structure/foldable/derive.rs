#[macro_export]
macro_rules! derive_foldable_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::structure::foldable::Foldable for $instance {
            fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                F: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<
                            A,
                            Output: $crate::base::function::ConcurrentFn<B, Output = B>,
                        >,
                    >,
                T: $crate::base::function::ConcurrentFn<A, Output = ::std::ops::ControlFlow<B, A>>,
            {
                match $crate::base::function::ConcurrentFn::call(&try_break, container.0) {
                    std::ops::ControlFlow::Break(res) => res,
                    std::ops::ControlFlow::Continue(x) => {
                        $crate::base::function::ConcurrentFn::call(
                            &$crate::base::function::ConcurrentFn::call(&accum.view(), x),
                            init,
                        )
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! derive_foldable_for_nested_foldable {
    ($instance:ty, $wrapper:ident, $inner_instance:ty) => {
        impl $crate::control::structure::foldable::Foldable for $instance {
            fn try_foldr<A, B, F, T>(accum: F, try_break: T, init: B, container: Self::Type<A>) -> B
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                F: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<
                            A,
                            Output: $crate::base::function::ConcurrentFn<B, Output = B>,
                        >,
                    >,
                T: $crate::base::function::ConcurrentFn<A, Output = ::std::ops::ControlFlow<B, A>>,
            {
                <$inner_instance as $crate::control::structure::foldable::Foldable>::try_foldr(
                    accum,
                    try_break,
                    init,
                    container.0,
                )
            }
        }
    };
}
