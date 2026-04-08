use crate::data::either::Either;
use crate::data::validation::Validation;

pub fn either_to_validation<E, A>(x: Either<E, A>) -> Validation<E, A> {
    x.into()
}

pub fn validation_to_either<E, A>(x: Validation<E, A>) -> Either<E, A> {
    x.into()
}

pub fn ealt<E, A>(one: Validation<E, A>, another: Validation<E, A>) -> Validation<E, A> {
    match &one {
        Validation::Success(_) => one,
        Validation::Failure(_) => another,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ealt() {
        let s1 = Validation::Success(1);
        let s2 = Validation::Success(2);
        let f1 = Validation::Failure("err1");
        let f2 = Validation::Failure("err2");

        assert_eq!(ealt(s1, s2), s1);
        assert_eq!(ealt(s1, f1), s1);
        assert_eq!(ealt(f1, s1), s1);
        assert_eq!(ealt(f1, f2), f2);
    }
}
