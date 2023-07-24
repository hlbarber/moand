use std::future::{ready, Future, Ready};

use futures_util::{future, FutureExt};

use crate::{FMap, Functor, Join, Monad, TypeConstructor};

pub struct _Future;

impl TypeConstructor for _Future {
    type Unit<T> = Ready<T>;

    fn unit<T>(item: T) -> Self::Unit<T> {
        ready(item)
    }
}

impl<Fut> Functor<_Future> for Fut
where
    Fut: Future,
{
    type Item = Fut::Output;
}

impl<Fut> Join<_Future> for Fut
where
    Fut: Future,
    Fut::Output: Future,
{
    type Joined = future::Flatten<Self>;

    fn join(self) -> Self::Joined {
        self.flatten()
    }
}

impl<Fut> Monad<_Future> for Fut where Fut: Future {}

impl<F, Fut, Output> FMap<_Future, Fut> for F
where
    Fut: Future,
    F: FnOnce(Fut::Output) -> Output,
{
    type Mapped = future::Map<Fut, F>;

    fn map(self, fut: Fut) -> Self::Mapped {
        FutureExt::map(fut, self)
    }
}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::{Monad, MonadExt};

    async fn _check() {
        let fut = async { async { 3 } };
        let _fut = fut.join().bind(|x| async move { x });
        let _output = _fut.await;
    }
}
