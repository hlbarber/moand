use crate::{FMap, Functor, Join, Monad, TypeConstructor};

pub struct _Option;

impl TypeConstructor for _Option {
    type Unit<T> = Option<T>;

    fn unit<S>(item: S) -> Self::Unit<S> {
        Some(item)
    }
}

impl<F, T, U> FMap<_Option, Option<T>> for F
where
    F: FnOnce(T) -> U,
{
    type Mapped = Option<U>;

    fn map(self, object: Option<T>) -> Self::Mapped {
        object.map(self)
    }
}

impl<T> Join<_Option> for Option<Option<T>> {
    type Joined = Option<T>;

    fn join(self) -> Self::Joined {
        self.flatten()
    }
}

impl<T> Functor<_Option> for Option<T> {
    type Item = T;
}

impl<T> Monad<_Option> for Option<T> {}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::{Monad, MonadExt};
    use ::std::option::Option::Some;

    fn _check() {
        let x = Some(Some("hey"));
        let x = x.bind(|x| Some(x));
        let _x = Monad::join(x);
    }
}
