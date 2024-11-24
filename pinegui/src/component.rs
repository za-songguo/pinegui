extern crate alloc;

use core::fmt;

use dyn_ord::DynEq;

use crate::{node::Node, shared::Shared, ui::Ui};

/// All components should implement this type.
pub trait Component: dyn_clone::DynClone + Send + fmt::Debug + DynEq {
    /// Returns the content to be rendered by the component.
    fn view(&mut self, ui: &Ui) -> Shared<Node>;
}

dyn_clone::clone_trait_object!(Component);

// TODO view 中先创建 Node，调用 view 后把它放进 node tree 之前先设置 component！！！！！
