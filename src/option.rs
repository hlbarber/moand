use crate::{Collection, Flatten, Functor, Member};

struct OptionMonad;

impl Collection for OptionMonad {
    type Unit<T> = Option<T>;

    fn unit<S>(item: S) -> Self::Unit<S> {
        Some(item)
    }
}

impl<T> Member<OptionMonad> for Option<T> {
    type Item = T;
}

impl<T> Flatten<OptionMonad> for Option<Option<T>> {
    type Flattened = Option<T>;

    fn flatten(self) -> Self::Flattened {
        Option::flatten(self)
    }
}

impl<T, U, F> Functor<OptionMonad, F> for Option<T>
where
    F: FnOnce(T) -> U,
{
    type Mapped = Option<U>;

    fn map(self, f: F) -> Self::Mapped {
        Option::map(self, f)
    }
}

#[cfg(test)]
mod tests {
    #![no_implicit_prelude]

    use crate::{Flatten, FunctorExt};
    use ::std::option::Option::Some;

    fn _check() {
        let x = Some(Some("hey"));
        let x = Flatten::flatten(x);
        let _y = x.flat_map(|_x| Some(3_u32)).map(|x| x);
    }
}
