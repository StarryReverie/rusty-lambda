#[macro_export]
macro_rules! derive_applicative_for_wrapper {
    ($instance:ty, $wrapper:ident) => {
        impl $crate::control::context::applicative::Applicative for $instance {
            fn pure<A>(x: A) -> Self::Type<A>
            where
                A: $crate::base::value::Value,
            {
                $wrapper(x)
            }

            fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = B>,
                    >,
            {
                $wrapper($crate::base::function::ConcurrentFn::call(&g.0.view(), x.0))
            }
        }

        impl<T> $crate::control::context::applicative::ApplicativeExt for $wrapper<T>
        where
            T: $crate::base::value::Value,
        {
            type Wrapped = T;
            type Instance = $instance;
        }
    };
}

#[macro_export]
macro_rules! derive_applicative_for_nested_applicative {
    ($instance:ty, $wrapper:ident, $inner_instance:ty) => {
        impl $crate::control::context::applicative::Applicative for $instance {
            fn pure<A>(x: A) -> Self::Type<A>
            where
                A: $crate::base::value::Value,
            {
                $wrapper(
                    <$inner_instance as $crate::control::context::applicative::Applicative>::pure(
                        x,
                    ),
                )
            }

            fn apply<A, B, G>(g: Self::Type<G>, x: Self::Type<A>) -> Self::Type<B>
            where
                A: $crate::base::value::Value,
                B: $crate::base::value::Value,
                G: for<'a> $crate::base::value::Value<
                        View<'a>: $crate::base::function::ConcurrentFn<A, Output = B>,
                    >,
            {
                $wrapper(
                    <$inner_instance as $crate::control::context::applicative::Applicative>::apply(
                        g.0, x.0,
                    ),
                )
            }
        }

        impl<T> $crate::control::context::applicative::ApplicativeExt for $wrapper<T>
        where
            T: $crate::base::value::Value,
        {
            type Wrapped = T;
            type Instance = $instance;
        }
    };
}
