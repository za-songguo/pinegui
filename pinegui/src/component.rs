extern crate alloc;

use core::{any::Any, fmt};

use crate::{node::Node, shared::Shared, ui::Ui};
use embedded_graphics::draw_target::DrawTarget;

/// All components should implement this type.
pub trait Component<T: DrawTarget>: dyn_clone::DynClone + Send + fmt::Debug {
    /// Returns the content to be rendered by the component.
    fn view(&mut self, ui: &Ui<T>) -> Shared<Node<T>>;

    fn as_any(&self) -> &dyn Any;
    fn is_eq(&self, other: &dyn Component<T>) -> bool;
}

dyn_clone::clone_trait_object!(<T> Component<T> where T: DrawTarget);

impl<T: DrawTarget> PartialEq for dyn Component<T> {
    fn eq(&self, other: &Self) -> bool {
        self.is_eq(other)
    }
}

// TODO view 中先创建 Node，调用 view 后把它放进 node tree 之前先设置 component！！！！！
