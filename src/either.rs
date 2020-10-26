pub use Either::{Left, Right};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[allow(dead_code)]
impl<L, R> Either<L, R> {

    fn is_left(&self) -> bool {
        match *self {
            Left(_) => true,
            Right(_) => false,
        }
    }

    fn is_right(&self) -> bool {
        !self.is_left()
    }

    fn left(self) -> Option<L> {
        match self {
            Left(l) => Some(l),
            Right(_) => None,
        }
    }

    fn right(self) -> Option<R> {
        match self {
            Left(_) => None,
            Right(r) => Some(r),
        }
    }

}

