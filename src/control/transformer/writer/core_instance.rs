use crate::base::function::{ConcurrentFn, Curry, WrappedFn};
use crate::base::value::Value;
use crate::control::context::applicative::{Applicative, ApplicativeExt};
use crate::control::context::monad::{Monad, MonadExt};
use crate::control::structure::functor::{Functor, FunctorExt};
use crate::control::structure::monoid::Monoid;
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT};

impl<W, M> Functor for StackedWriterTInstance<W, M>
where
    W: Value,
    M: Functor,
{
    fn fmap<A, B, G>(g: G, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        WriterT::new(M::fmap(
            WrappedFn::from(move |(x, log)| (g.clone().view().call(x), log)),
            WriterT::run_tr(fx),
        ))
    }
}

impl<W, M, A> FunctorExt for WriterT<W, M, A>
where
    W: Value,
    M: Functor,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedWriterTInstance<W, M>;
}

impl<W, M> Applicative for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: Applicative,
{
    fn pure<A>(x: A) -> Self::Type<A>
    where
        A: Value,
    {
        WriterT::new(M::pure((x, W::empty())))
    }

    fn apply<A, B, G>(fg: Self::Type<G>, fx: Self::Type<A>) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = B>>,
    {
        WriterT::new(M::apply(
            M::fmap(
                WrappedFn::curry(move |(g, log1): (G, W), (x, log2): (A, W)| {
                    (g.view().call(x), log1.associate(log2))
                }),
                WriterT::run_tr(fg),
            ),
            WriterT::run_tr(fx),
        ))
    }
}

impl<W, M, A> ApplicativeExt for WriterT<W, M, A>
where
    W: Monoid + Value,
    M: Applicative,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedWriterTInstance<W, M>;
}

impl<W, M> Monad for StackedWriterTInstance<W, M>
where
    W: Monoid + Value,
    M: Monad,
{
    fn bind<A, B, G>(mx: Self::Type<A>, g: G) -> Self::Type<B>
    where
        A: Value,
        B: Value,
        G: for<'a> Value<View<'a>: ConcurrentFn<A, Output = Self::Type<B>>>,
    {
        WriterT::new(M::bind(
            WriterT::run_tr(mx),
            WrappedFn::from(move |(x, log1): (A, W)| {
                let my = WriterT::run_tr(g.view().call(x));
                M::fmap(
                    WrappedFn::from(move |(y, log2)| (y, log1.clone().associate(log2))),
                    my,
                )
            }),
        ))
    }
}

impl<W, M, A> MonadExt for WriterT<W, M, A>
where
    W: Monoid + Value,
    M: Monad,
    A: Value,
{
    type Wrapped = A;
    type Instance = StackedWriterTInstance<W, M>;
}

#[cfg(test)]
mod tests {
    use crate::base::function::{ConcurrentFn, Curry, WrappedFn, compose};
    use crate::control::context::applicative::{Applicative, ApplicativeExt};
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::control::structure::functor::fmap;
    use crate::control::transformer::writer::{Writer, WriterInstance};
    use crate::data::list::List;

    #[test]
    fn writer_functor_identity_law() {
        let writer = Writer::from((10, List::from(vec![1, 2])));
        let mapped = fmap(WrappedFn::from(|x| x), writer.clone());
        assert_eq!(Writer::run(mapped), Writer::run(writer));
    }

    #[test]
    fn writer_functor_composition_law() {
        let g = WrappedFn::from(|x| x + 3);
        let h = WrappedFn::from(|x| x * 2);
        let composed = g.clone().compose(h.clone());

        let writer = Writer::from((5, List::from(vec![1])));
        let lhs = fmap(composed, writer.clone());
        let rhs = fmap(g, fmap(h, writer));
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }

    #[test]
    fn writer_applicative_identity_law() {
        let writer = Writer::from((10, List::from(vec![1, 2])));
        let writer = WriterInstance::pure(WrappedFn::from(|x| x)).apply(writer);
        let (val, log) = Writer::run(writer);
        assert_eq!(val, 10);
        assert_eq!(log, List::from(vec![1, 2]));
    }

    #[test]
    fn writer_applicative_homomorphism_law() {
        let g = WrappedFn::from(|x| x * 2);
        let lhs: Writer<List<i32>, i32> =
            WriterInstance::pure(g.clone()).apply(WriterInstance::pure(3));
        let rhs: Writer<List<i32>, i32> = WriterInstance::pure(g(3));
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }

    #[test]
    fn writer_applicative_interchange_law() {
        let g = Writer::from((WrappedFn::from(|x| x + 10), List::from(vec![7])));
        let x = 5;

        let lhs = g.clone().apply(WriterInstance::pure(x));
        let rhs =
            WriterInstance::pure(WrappedFn::from(move |g: WrappedFn<i32, i32>| g(x))).apply(g);
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }

    #[test]
    fn writer_applicative_composition_law() {
        let g = Writer::from((WrappedFn::from(|x| x * 2), List::from(vec![1])));
        let h = Writer::from((WrappedFn::from(|x| x + 3), List::from(vec![2])));
        let composed = WriterInstance::pure(WrappedFn::curry(compose))
            .apply(g.clone())
            .apply(h.clone());

        let x = WriterInstance::pure(4);
        let lhs = composed.apply(x.clone());
        let rhs = g.apply(h.apply(x));
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }

    #[test]
    fn writer_monad_left_identity_law() {
        let g = WrappedFn::from(|x| Writer::from((x * 2, List::from(vec![x]))));
        let lhs = WriterInstance::ret(3).bind(g.clone());
        let rhs = g(3);
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }

    #[test]
    fn writer_monad_right_identity_law() {
        let writer = Writer::from((7, List::from(vec![1, 2])));
        let (x, log) = Writer::run(writer.bind(WrappedFn::from(|x| WriterInstance::ret(x))));
        assert_eq!(x, 7);
        assert_eq!(log, List::from(vec![1, 2]));
    }

    #[test]
    fn writer_monad_associativity_law() {
        let g = WrappedFn::from(|x| Writer::from((x + 1, List::from(vec![x]))));
        let h = WrappedFn::from(|x| Writer::from((x * 2, List::from(vec![100]))));

        let writer = Writer::from((5, List::from(vec![1])));
        let lhs = writer.clone().bind(g.clone()).bind(h.clone());
        let rhs = writer.bind(WrappedFn::from(move |x| g(x).bind(h.clone())));
        assert_eq!(Writer::run(lhs), Writer::run(rhs));
    }
}
