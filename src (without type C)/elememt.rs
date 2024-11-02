use embedded_graphics::{
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};
use pinegui_event::Event;
use pinegui_renderer::render::Render;

extern crate alloc;
use alloc::{boxed::Box, sync::Arc};

pub struct Element<T: DrawTarget, C: FnOnce(Event)> {
    pub content: Box<dyn Render<T>>,
    pub event_callback: C,
}

impl<T: DrawTarget, C: FnOnce(Event)> Element<T, C> {
    pub fn new(content: Box<dyn Render<T>>) -> Self {
        Self {
            content,
            event_callback: Arc::new(|_| {}),
        }
    }

    pub fn set_content(&mut self, content: Box<dyn Render<T>>) -> Self {
        self.content = content;
        self.clone()
    }

    pub fn set_event_callback<C>(&mut self, event_callback: Arc<dyn FnOnce(Event)>) -> Self {
        self.event_callback = event_callback;
        self.clone()
    }
}

impl<T: DrawTarget> Dimensions for Element<T> {
    fn bounding_box(&self) -> Rectangle {
        self.content.bounding_box()
    }
}

impl<T: DrawTarget> PartialEq for Element<T> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content.clone()
    }
}

impl<T: DrawTarget> Clone for Element<T> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            event_callback: self.event_callback.clone(),
        }
    }
}
