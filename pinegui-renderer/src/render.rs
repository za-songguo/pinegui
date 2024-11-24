use core::fmt;
use dyn_clone::DynClone;
use dyn_ord::DynEq;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::Drawable;

extern crate alloc;

// A marker trait.
pub trait Render: fmt::Debug + DynEq + DynClone + Dimensions {}

dyn_clone::clone_trait_object!(Render);

impl<T: fmt::Debug + DynEq + DynClone + Dimensions + Drawable> Render for T {}

impl PartialEq for dyn Render {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any().downcast_ref::<&dyn DynEq>().unwrap())
    }
}
