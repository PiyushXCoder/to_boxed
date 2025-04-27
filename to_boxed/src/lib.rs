pub use to_boxed_derive::*;

pub trait ToBoxed {
    fn to_boxed(self) -> Box<Self>
    where
        Self: Sized;
}
