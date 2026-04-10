use crate::base::value::Value;
use crate::control::structure::functor::identity::{Identity, IdentityInstance};
use crate::control::transformer::writer::{StackedWriterTInstance, WriterT};

pub type Writer<W, A> = WriterT<W, IdentityInstance, A>;
pub type WriterInstance<W> = StackedWriterTInstance<W, IdentityInstance>;

impl<W, A> Writer<W, A>
where
    W: Value,
    A: Value,
{
    pub fn run(writer: Self) -> (A, W) {
        Identity::run(WriterT::run_tr(writer))
    }

    pub fn eval(writer: Self) -> A {
        Self::run(writer).0
    }

    pub fn exec(writer: Self) -> W {
        Self::run(writer).1
    }
}

#[cfg(test)]
mod tests {
    use crate::control::context::monad::{Monad, MonadExt};
    use crate::data::list::List;

    use super::*;

    #[test]
    fn test_run() {
        let writer = Writer::from((42, List::from(vec![1, 2, 3])));
        assert_eq!(Writer::run(writer), (42, List::from(vec![1, 2, 3])));
    }

    #[test]
    fn test_eval() {
        let writer = Writer::from((42, List::from(vec![1, 2, 3])));
        assert_eq!(Writer::eval(writer), 42);
    }

    #[test]
    fn test_exec() {
        let writer = Writer::from((42, List::from(vec![1, 2, 3])));
        assert_eq!(Writer::exec(writer), List::from(vec![1, 2, 3]));
    }

    #[test]
    fn test_bind_accumulates_log() {
        let writer = Writer::from((10, List::from(vec![1])));
        let writer = writer.bind(&|x| Writer::from((x * 2, List::from(vec![2, 3]))));
        assert_eq!(Writer::run(writer), (20, List::from(vec![1, 2, 3])));
    }

    #[test]
    fn test_pure_empty_log() {
        let writer: Writer<List<i32>, _> = WriterInstance::ret(5);
        assert_eq!(Writer::run(writer), (5, List::from(vec![])));
    }
}
