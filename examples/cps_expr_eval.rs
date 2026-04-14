use rusty_lambda::base::computation::Computation;

enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    fn eval(self) -> i64 {
        Self::eval_cps(self, Box::new(|v| Computation::immediate(v))).eval()
    }

    #[rustfmt::skip]
    fn eval_cps(
        expr: Self,
        cont: Box<dyn FnOnce(i64) -> Computation<i64> + Send + Sync + 'static>,
    ) -> Computation<i64> {
        match expr {
            Self::Num(n) => Computation::monadic(move || cont(n)),
            Self::Add(left, right) => Computation::monadic(move || {
                Self::eval_cps(*left, Box::new(move |lv| Computation::monadic(move || {
                    Self::eval_cps(*right, Box::new(move |rv| Computation::monadic(move || {
                        cont(lv + rv)
                    })))
                })))
            }),
            Self::Mul(left, right) => Computation::monadic(move || {
                Self::eval_cps(*left, Box::new(move |lv| Computation::monadic(move || {
                    Self::eval_cps(*right, Box::new(move |rv| Computation::monadic(move || {
                        cont(lv * rv)
                    })))
                })))
            }),
            Self::Neg(inner) => Computation::monadic(move || {
                Self::eval_cps(*inner, Box::new(move |v| Computation::monadic(move || {
                    cont(-v)
                })))
            }),
        }
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
            Box::new(Expr::Add(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
        )),
        Box::new(Expr::Neg(Box::new(Expr::Mul(
            Box::new(Expr::Num(5)),
            Box::new(Expr::Num(6)),
        )))),
    );
    println!("(1 + 2) * (3 + 4) + -(5 * 6) = {}", expr.eval());

    let expr = (1..=100000).fold(Expr::Num(0), |acc, i| {
        Expr::Add(Box::new(acc), Box::new(Expr::Num(i)))
    });
    println!("((((1 + 2) + 3) + ...) + 1000000 = {}", expr.eval());
}
