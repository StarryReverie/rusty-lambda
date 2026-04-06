#[macro_export]
macro_rules! derive_monad_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::context::monad::Monad for $instance {
            fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = Self::Type<B>>,
                    >,
            {
                $crate::base::function::ConcurrentFn::call(&g.view(), x.0)
            }
        }

        impl<T> $crate::control::context::monad::MonadExt for $wrapper<T>
        where
            T: $crate::base::value::StaticConcurrent,
        {
            type Wrapped = T;
            type Instance = $instance;
        }
    };
}

#[macro_export]
macro_rules! derive_monad_for_nested_monad {
    ($instance:ty, $wrapper:ident, $inner_instance:ty) => {
        impl $crate::control::context::monad::Monad for $instance {
            fn bind<A, B, G>(x: Self::Type<A>, g: G) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = Self::Type<B>>,
                    >,
            {
                $wrapper(
                    <$inner_instance as $crate::control::context::monad::Monad>::bind(
                        x.0,
                        $crate::base::function::WrappedFn::from(move |x| {
                            $crate::base::function::ConcurrentFn::call(&g.view(), x).0
                        }),
                    ),
                )
            }
        }

        impl<T> $crate::control::context::monad::MonadExt for $wrapper<T>
        where
            T: $crate::base::value::StaticConcurrent,
        {
            type Wrapped = T;
            type Instance = $instance;
        }
    };
}
