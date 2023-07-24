mod future;
mod iterator;
mod option;

pub use future::*;
pub use iterator::*;
pub use option::*;

/// A marker trait for a collections.
pub trait Collection: Sized {
    type Unit<T>: Member<Self, Item = T>;

    fn unit<T>(item: T) -> Self::Unit<T>;
}

/// A marker trait for a member of a collection.
pub trait Member<C: Collection> {
    type Item;
}

/// Provides a `map` function for items in a collection.
pub trait Functor<C, F>: Member<C>
where
    C: Collection,
    F: FnOnce(Self::Item) -> <Self::Mapped as Member<C>>::Item,
{
    type Mapped: Member<C>;

    fn map(self, f: F) -> Self::Mapped;
}

/// The flatten operation on monads.
pub trait Flatten<M>: Member<M>
where
    Self::Item: Member<M>,
    M: Collection,
{
    type Flattened: Member<M, Item = <Self::Item as Member<M>>::Item>;

    fn flatten(self) -> Self::Flattened;
}

pub trait FunctorExt<C, F>: Functor<C, F>
where
    C: Collection,
    F: FnOnce(Self::Item) -> <Self::Mapped as Member<C>>::Item,
{
    fn flat_map(self, f: F) -> <Self::Mapped as Flatten<C>>::Flattened
    where
        <Self::Mapped as Member<C>>::Item: Member<C>,
        Self::Mapped: Flatten<C>,
        Self: Sized,
    {
        Functor::map(self, f).flatten()
    }
}

impl<C, F, T> FunctorExt<C, F> for T
where
    T: Functor<C, F>,
    F: FnOnce(Self::Item) -> <Self::Mapped as Member<C>>::Item,
    C: Collection,
{
}

#[cfg(test)]
mod tests {
    fn _check<M: () {

    }
}
