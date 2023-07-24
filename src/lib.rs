mod future;
mod iterator;
mod option;

pub use future::*;
pub use iterator::*;
pub use option::*;

/// A marker trait for the type constructor of a collection.
pub trait TypeConstructor: Sized {
    type Unit<T>: Functor<Self, Item = T>;

    fn unit<T>(item: T) -> Self::Unit<T>;
}

pub trait FMap<C, Object>
where
    Object: Functor<C>,
{
    type Mapped: Functor<C>;

    fn map(self, object: Object) -> Self::Mapped;
}

/// A member of a collection.
pub trait Functor<C> {
    /// The inner collection type.
    type Item;

    /// Applies a map to a collection.
    fn map<F>(self, f: F) -> F::Mapped
    where
        F: FMap<C, Self>,
        Self: Sized,
    {
        FMap::map(f, self)
    }
}

pub trait Join<C> {
    type Joined: Functor<C>;

    fn join(self) -> Self::Joined;
}

/// A member of a monad.
pub trait Monad<C>: Functor<C> {
    fn join(self) -> Self::Joined
    where
        Self: Join<C>,
        Self: Sized,
    {
        Join::join(self)
    }
}

pub trait MonadExt<C>: Monad<C>
where
    C: TypeConstructor,
{
    fn bind<F>(self, f: F) -> <F::Mapped as Join<C>>::Joined
    where
        F: FMap<C, Self>,
        F::Mapped: Join<C>,
        Self: Sized,
    {
        self.map(f).join()
    }
}

impl<C, T> MonadExt<C> for T
where
    C: TypeConstructor,
    T: Monad<C>,
{
}

#[macro_export]
macro_rules! du {
    ($binding:ident <- $e:expr ; $($t:tt)*) => {
        $e.bind(move |$binding| { du! { $($t)* } })
    };
    ($binding:pat = $e:expr ; $($t:tt)*) => {
        {
            let $binding = $e;
            du!($($t)*)
        }
    };
    ($final:expr) => { $final };
}

#[cfg(test)]
mod tests {
    use crate::{MonadExt, TypeConstructor, _Future};

    fn _concat() {
        fn foo() -> <M as TypeConstructor>::Unit<u32> {
            M::unit(3)
        }

        type M = _Future;
        let _result = du! {
            x <- foo();
            y <- M::unit(2);
            z = M::unit(x + y);
            z
        };
    }
}
