extern crate alloc;

use core::fmt;

use alloc::boxed::Box;

use embedded_graphics::{
    prelude::{Dimensions, DrawTarget},
    primitives::Rectangle,
};
use pinegui_event::Event;
use pinegui_renderer::render::Render;

use crate::shared::Shared;

pub struct Element<T: DrawTarget> {
    pub content: Box<dyn Render<T>>,
    pub event_callback: Shared<Box<dyn Fn(Event) + Send>>,
}

impl<T: DrawTarget> Element<T> {
    pub fn new(
        content: Box<dyn Render<T>>,
        event_callback: Shared<Box<dyn Fn(Event) + Send>>,
    ) -> Self {
        Self {
            content,
            event_callback,
        }
    }

    pub fn set_content(&mut self, content: Box<dyn Render<T>>) -> Self {
        self.content = content;
        self.clone()
    }

    pub fn set_event_callback(
        &mut self,
        event_callback: Shared<Box<dyn Fn(Event) + Send>>,
    ) -> Self {
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
        // XXX: Is comparing pointer addresses enough?
        // let callback_is_eq = SmartRc::ptr_eq(
        //     &self.event_callback.get_inner(),
        //     &other.event_callback.get_inner(),
        // );

        // self.content == other.content.clone() && callback_is_eq
        self.content == other.content.clone()
    }
}

impl<T: DrawTarget> Eq for Element<T> {}

impl<T: DrawTarget> Clone for Element<T> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            event_callback: self.event_callback.clone(),
        }
    }
}

impl<T: DrawTarget> fmt::Debug for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Element")
            .field("content", &self.content)
            .field("event_callback", &"Callback snipped")
            .finish()
    }
}
