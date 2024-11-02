use core::{any::Any, fmt};

use embedded_graphics::{
    pixelcolor::PixelColor,
    prelude::{Dimensions, DrawTarget},
    Drawable,
};

extern crate alloc;

// // 定义一个标记 trait `Render2`
// pub trait Render2<T>: dyn_clone::DynClone + Dimensions + Send + fmt::Debug + Drawable
// where
//     T: DrawTarget,
// {
// }

// // 使用标记 trait 的具体实现
// impl<C: PixelColor, T: DrawTarget<Color = C>, D> Render2<T> for D where
//     D: Drawable<Color = C> + Dimensions + Clone + PartialEq + 'static + Send + fmt::Debug
// {
// }

// pub trait Render<T: DrawTarget>: Send + Sync + DynClone {
// TODO why Dimensions trait is needed here?
pub trait Render<T>: dyn_clone::DynClone + Dimensions + Send + fmt::Debug
where
    T: DrawTarget,
{
    fn render(&self, target: &mut T) -> Result<(), T::Error>;

    fn as_any(&self) -> &dyn Any;
    fn is_eq(&self, other: &dyn Render<T>) -> bool;
}

dyn_clone::clone_trait_object!(<T> Render<T> where T: DrawTarget);

impl<C: PixelColor, T: DrawTarget<Color = C>, D> Render<T> for D
where
    D: Drawable<Color = C> + Dimensions + Clone + PartialEq + 'static + Send + fmt::Debug,
{
    fn render(&self, target: &mut T) -> Result<(), T::Error> {
        self.draw(target)?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_eq(&self, other: &dyn Render<T>) -> bool {
        if let Some(other_concrete) = other.as_any().downcast_ref::<Self>() {
            self == other_concrete
        } else {
            false
        }
    }
}

impl<C: PixelColor, T: DrawTarget<Color = C>> PartialEq for dyn Render<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_eq(other)
    }
}

impl<C: PixelColor, T: DrawTarget<Color = C>> Eq for dyn Render<T> {}

// impl<C: PixelColor, T: DrawTarget<Color = C>> fmt::Debug for dyn Render<T>
// where
//     Self: fmt::Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let debug = self.as_any().downcast_ref::<&dyn fmt::Debug>().unwrap();
//         write!(f, "{:?}", debug)
//     }
// }

// /// `RenderGroup` contains a series of types that implement `Drawable` to render multiple of those types at once.
// ///
// /// This type also implements `Drawable`.
// #[derive(Clone, PartialEq, Eq)]
// pub struct RenderGroup<C: PixelColor, D>
// where
//     D: Drawable<Color = C> + Clone + PartialEq, // + Send + Sync
// {
//     items: Vec<D>,
// }

// impl<C: PixelColor, D> RenderGroup<C, D>
// where
//     D: Drawable<Color = C> + Clone + PartialEq,
// {
//     /// Create a new `RenderGroup` with some renderable items in it.
//     pub fn new(items: Vec<D>) -> Self {
//         Self { items }
//     }

//     /// Returns a reference to the items in the group.
//     pub fn items(&self) -> &Vec<D> {
//         &self.items
//     }

//     /// Returns a mutable reference to the items in the group.
//     pub fn items_mut(&mut self) -> &mut Vec<D> {
//         &mut self.items
//     }
// }

// impl<C: PixelColor, D> Drawable for RenderGroup<C, D>
// where
//     D: Drawable<Color = C> + Clone + PartialEq,
// {
//     type Output = ();
//     type Color = C;

//     fn draw<T>(&self, target: &mut T) -> Result<Self::Output, T::Error>
//     where
//         T: DrawTarget<Color = Self::Color>,
//     {
//         for i in self.items() {
//             i.draw(target)?;
//         }
//         Ok(())
//     }
// }
