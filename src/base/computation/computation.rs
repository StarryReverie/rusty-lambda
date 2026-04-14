use crate::base::value::StaticConcurrent;

pub type LazyReturn<T> = Box<dyn FnOnce() -> T + Send + Sync + 'static>;
pub type LazyComputation<T> = LazyReturn<Computation<T>>;

pub enum Computation<T> {
    Evaluated(T),
    Deferred(LazyComputation<T>),
}

impl<T> Computation<T> {
    pub fn immediate(value: T) -> Self {
        Self::Evaluated(value)
    }

    pub fn lazy<F>(eval: F) -> Self
    where
        F: FnOnce() -> T + StaticConcurrent,
    {
        Self::Deferred(Box::new(move || Self::Evaluated(eval())))
    }

    pub fn monadic<F>(eval: F) -> Self
    where
        F: FnOnce() -> Self + StaticConcurrent,
    {
        Self::Deferred(Box::new(eval))
    }

    pub fn eval(self) -> T {
        let mut current = self;
        loop {
            match current {
                Self::Evaluated(value) => return value,
                Self::Deferred(next) => current = next(),
            }
        }
    }

    pub fn lazy_eval(self) -> LazyReturn<T>
    where
        T: StaticConcurrent,
    {
        match self {
            Self::Evaluated(value) => Box::new(move || value),
            computation @ Self::Deferred(_) => Box::new(move || computation.eval()),
        }
    }

    pub fn map<U, F>(self, transform: F) -> Computation<U>
    where
        T: StaticConcurrent,
        F: FnOnce(T) -> U + StaticConcurrent,
    {
        Computation::lazy(move || transform(self.eval()))
    }

    pub fn bind<U, F>(self, next: F) -> Computation<U>
    where
        T: StaticConcurrent,
        F: FnOnce(T) -> Computation<U> + StaticConcurrent,
    {
        Computation::monadic(move || next(self.eval()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_tail_call_optimization() {
        fn factorial_tail(x: u32) -> u32 {
            fn go(x: u32, res: u32) -> Computation<u32> {
                if x == 0 {
                    Computation::immediate(res)
                } else {
                    Computation::monadic(move || go(x - 1, res * x))
                }
            }
            go(x, 1).eval()
        }

        assert_eq!(factorial_tail(0), 1);
        assert_eq!(factorial_tail(1), 1);
        assert_eq!(factorial_tail(2), 2);
        assert_eq!(factorial_tail(3), 6);
        assert_eq!(factorial_tail(4), 24);
    }

    #[test]
    fn test_tree_sum_cps() {
        let tree = Box::new(Tree::Node(
            1,
            Box::new(Tree::Node(
                2,
                Box::new(Tree::Node(4, Box::new(Tree::Nil), Box::new(Tree::Nil))),
                Box::new(Tree::Node(5, Box::new(Tree::Nil), Box::new(Tree::Nil))),
            )),
            Box::new(Tree::Node(
                3,
                Box::new(Tree::Node(6, Box::new(Tree::Nil), Box::new(Tree::Nil))),
                Box::new(Tree::Nil),
            )),
        ));

        let cont = Box::new(|x| Computation::immediate(x));
        assert_eq!(sum_cps(tree, cont).eval(), 21);
    }

    #[test]
    fn test_tree_sum_cps_no_stack_overflow() {
        let n = 100000;
        let tree = (1..=n).fold(Box::new(Tree::Nil), |acc, i| {
            Box::new(Tree::Node(i, acc, Box::new(Tree::Nil)))
        });

        let cont = Box::new(|x| Computation::immediate(x));
        assert_eq!(sum_cps(tree, cont).eval(), (1 + n) * n / 2);
    }

    enum Tree {
        Nil,
        Node(u64, Box<Tree>, Box<Tree>),
    }

    #[rustfmt::skip]
    fn sum_cps(
        node: Box<Tree>,
        cont: Box<dyn FnOnce(u64) -> Computation<u64> + Send + Sync + 'static>,
    ) -> Computation<u64> {
        match *node {
            Tree::Nil => Computation::monadic(move || cont(0)),
            Tree::Node(x, left, right) => Computation::monadic(move || {
                sum_cps(left, Box::new(move |ls| Computation::monadic(move || {
                    sum_cps(right, Box::new(move |rs| Computation::monadic(move || {
                        cont(ls + rs + x)
                    })))
                })))
            }),
        }
    }
}
