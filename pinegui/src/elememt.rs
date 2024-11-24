extern crate alloc;

use core::fmt;

use alloc::boxed::Box;

use embedded_graphics::{prelude::Dimensions, primitives::Rectangle};
use pinegui_event::Event;
use pinegui_renderer::render::Render;

use crate::shared::Shared;

pub struct Element {
    pub content: Box<dyn Render>,
    pub event_callback: Shared<Box<dyn Fn(Event) + Send>>,
}

impl Element {
    pub fn new(
        content: Box<dyn Render>,
        event_callback: Shared<Box<dyn Fn(Event) + Send>>,
    ) -> Self {
        Self {
            content,
            event_callback,
        }
    }

    pub fn set_content(&mut self, content: Box<dyn Render>) -> Self {
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

impl Dimensions for Element {
    fn bounding_box(&self) -> Rectangle {
        self.content.bounding_box()
    }
}

impl PartialEq for Element {
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

impl Eq for Element {}

impl Clone for Element {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            event_callback: self.event_callback.clone(),
        }
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Element")
            .field("content", &self.content)
            .field("event_callback", &"Callback snipped")
            .finish()
    }
}
