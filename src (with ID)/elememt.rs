use embedded_graphics::{
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};
use pinegui_event::Event;
use pinegui_renderer::render::Render;

extern crate alloc;
use alloc::boxed::Box;

pub struct Element<T: DrawTarget, C: FnOnce(Event) + Clone> {
    pub content: Box<dyn Render<T>>,
    pub event_callback: C,
}

impl<T: DrawTarget, C: FnOnce(Event) + Clone> Element<T, C> {
    pub fn new(content: Box<dyn Render<T>>, event_callback: C) -> Self {
        Self {
            content,
            event_callback,
        }
    }

    pub fn set_content(&mut self, content: Box<dyn Render<T>>) -> Self {
        self.content = content;
        self.clone()
    }

    pub fn set_event_callback(&mut self, event_callback: C) -> Self {
        self.event_callback = event_callback;
        self.clone()
    }
}

impl<T: DrawTarget, C: FnOnce(Event) + Clone> Dimensions for Element<T, C> {
    fn bounding_box(&self) -> Rectangle {
        self.content.bounding_box()
    }
}

impl<T: DrawTarget, C: FnOnce(Event) + Clone> PartialEq for Element<T, C> {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content.clone()
    }
}

impl<T: DrawTarget, C: FnOnce(Event) + Clone> Clone for Element<T, C> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            event_callback: self.event_callback.clone(),
        }
    }
}
