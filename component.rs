use dyn_clone::DynClone;
use embedded_graphics::draw_target::DrawTarget;

extern crate alloc;
use alloc::boxed::Box;
use pinegui_renderer::render::RenderableList;

/// All components should implement this type.
pub trait Component<T: DrawTarget>: Send + Sync + DynClone {
    /// Returns the content to be rendered by the component.
    fn view(&self) -> RenderableList;
    fn id(&self) -> usize;
}

dyn_clone::clone_trait_object!(<T> Component<T> where T: DrawTarget);

impl<T: DrawTarget, C: Component<T>> Component<T> for &C {
    fn view(&self) -> RenderGroup<C, D> {
        (*self).view()
    }

    fn id(&self) -> usize {
        (*self).id()
    }
}

// TODO （抛弃 id，比较 content）
impl<T: DrawTarget> PartialEq for Box<dyn Component<T>> {
    fn eq(&self, other: &Box<dyn Component<T>>) -> bool {
        self.id() == other.id()
    }
}

impl<T: DrawTarget> Eq for Box<dyn Component<T>> {}
