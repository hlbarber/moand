use std::iter::{self, once, Once};

use crate::{FMap, Functor, Join, Monad, TypeConstructor};

pub struct _Iterator;

impl TypeConstructor for _Iterator {
    type Unit<T> = Once<T>;

    fn unit<T>(item: T) -> Self::Unit<T> {
        once(item)
    }
}

impl<It, F, Output> FMap<_Iterator, It> for F
where
    It: Iterator,
    F: FnMut(It::Item) -> Output,
{
    type Mapped = iter::Map<It, F>;

    fn map(self, monad: It) -> Self::Mapped {
        Iterator::map(monad, self)
    }
}

impl<It, T> Functor<_Iterator> for It
where
    It: Iterator<Item = T>,
{
    type Item = T;
}

impl<It, T> Join<_Iterator> for It
where
    It: Iterator,
    It::Item: Iterator<Item = T>,
{
    type Joined = std::iter::Flatten<It>;

    fn join(self) -> Self::Joined {
        Iterator::flatten(self)
    }
}

impl<It> Monad<_Iterator> for It where It: Iterator {}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::{Monad, MonadExt};
    use ::std::iter::IntoIterator;

    fn _check() {
        let x = [["a", "b"].into_iter(), ["c", "d"].into_iter()].into_iter();
        let _y = x.join().bind(|x| ["e", x].into_iter());
    }
}
