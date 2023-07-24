use std::iter::{self, once, Once};

use crate::{Collection, Flatten, Functor, Member};

struct IteratorMonad;

impl Collection for IteratorMonad {
    type Unit<T> = Once<T>;

    fn unit<T>(item: T) -> Self::Unit<T> {
        once(item)
    }
}

impl<It, T> Member<IteratorMonad> for It
where
    It: Iterator<Item = T>,
{
    type Item = T;
}

impl<It, T> Flatten<IteratorMonad> for It
where
    It: Iterator,
    It::Item: Iterator<Item = T>,
{
    type Flattened = std::iter::Flatten<It>;

    fn flatten(self) -> Self::Flattened {
        Iterator::flatten(self)
    }
}

impl<F, It, Output> Functor<IteratorMonad, F> for It
where
    It: Iterator,
    F: FnMut(It::Item) -> Output,
{
    type Mapped = iter::Map<It, F>;

    fn map(self, f: F) -> Self::Mapped {
        Iterator::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::*;
    use ::std::iter::IntoIterator;

    fn _check() {
        let x = [["a", "b"].into_iter(), ["c", "d"].into_iter()].into_iter();
        let _y = x.flatten();
    }
}
