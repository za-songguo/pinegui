extern crate alloc;

use alloc::vec::Vec;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry,
    prelude::*,
    primitives::{self, Rectangle},
};
use pinegui_event::{Event, EventPosition};

use crate::{component::Component, elememt::Element};

#[derive(Clone, PartialEq)]
pub struct Ui<T: DrawTarget> {
    pub width: u32,
    pub height: u32,
    pub top_left: Point,

    pub render_list: Vec<Element<T>>,
    pub child_comp: Component<T>,
}

impl<T: DrawTarget> Ui<T> {
    pub fn new(width: u32, height: u32, top_left: Point, child_comp: Component<T>) -> Self {
        Self {
            width,
            height,
            top_left,
            render_list: child_comp.expand(), // When creating the Ui, all the required components need to be rendered.
            child_comp,
        }
    }

    pub fn clear_render_list(&mut self) {
        self.render_list.clear();
    }

    pub fn dispatch_event(&self, event: Event) {
        for element in self.child_comp.expand() {
            // Dispatch the event to the element if the event is triggered on the element or if it is a global event.
            match event.position() {
                Some(position) => {
                    if element.content.bounding_box().contains(position) {
                        (element.event_callback)(event.clone());
                    }
                }
                // Global event
                None => {
                    (element.event_callback)(event.clone());
                }
            }
        }
    }
}

impl<T: DrawTarget> geometry::Dimensions for Ui<T> {
    fn bounding_box(&self) -> primitives::Rectangle {
        Rectangle::new(self.top_left, Size::new(self.width, self.height))
    }
}
