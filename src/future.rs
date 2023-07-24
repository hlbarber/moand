use std::future::{ready, Future, Ready};

use futures_util::{future, FutureExt};

use crate::{Collection, Flatten, Functor, Member};

pub struct FutureMonad;

impl Collection for FutureMonad {
    type Unit<T> = Ready<T>;

    fn unit<T>(item: T) -> Self::Unit<T> {
        ready(item)
    }
}

impl<Fut> Member<FutureMonad> for Fut
where
    Fut: Future,
{
    type Item = Fut::Output;
}

impl<Fut> Flatten<FutureMonad> for Fut
where
    Fut: Future,
    Fut::Output: Future,
{
    type Flattened = future::Flatten<Self>;

    fn flatten(self) -> Self::Flattened {
        future::FutureExt::flatten(self)
    }
}

impl<F, Fut, Output> Functor<FutureMonad, F> for Fut
where
    Fut: Future,
    F: FnOnce(Fut::Output) -> Output,
{
    type Mapped = future::Map<Fut, F>;

    fn map(self, f: F) -> Self::Mapped {
        FutureExt::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::{Flatten, FunctorExt};

    fn _check() {
        let fut = async { async { "hey" } };
        let _ = fut.flatten().flat_map(|x| async move { x });
    }
}
