extern crate alloc;

use core::sync::atomic::Ordering;

use crate::{
    elememt::Element,
    id::{ItemID, NEXT_ID},
};
use alloc::vec::Vec;
use embedded_graphics::draw_target::DrawTarget;

#[derive(Default)]
pub struct Component<T: DrawTarget> {
    pub id: usize,
    pub children: Vec<Component<T>>,
    pub content: Option<Vec<Element<T>>>,
}

impl<T: DrawTarget> Clone for Component<T> {
    fn clone(&self) -> Self {
        // dyn_clone::clone(self)
        Self {
            id: self.id,
            children: self.children.clone(),
            content: self.content.clone(),
        }
    }
}

impl<T: DrawTarget> Component<T> {
    pub fn new() -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            children: Vec::new(),
            content: None,
        }
    }

    pub fn add_child(&mut self, child: Component<T>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn set_content(&mut self, content: Option<Vec<Element<T>>>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn expand(&self) -> Vec<Element<T>> {
        let mut result = Vec::new();
        if let Some(c) = self.content.clone() {
            result.extend(c);
        }
        for child in &self.children {
            result.extend(child.expand());
        }
        result
    }

    pub fn find_comp_by_id(&self, id: usize) -> Option<&Component<T>> {
        let mut stack = vec![self];

        while let Some(node) = stack.pop() {
            if node.id == id {
                return Some(node);
            }

            for child in &node.children {
                if child.id == id {
                    return Some(child);
                }
                stack.push(child);
            }
        }
        None
    }

    pub fn clone_with_new_id(&self) -> Self {
        // The process will generate a new ID.
        let mut result = Self::new();

        result.children = self.children.clone();
        result.content = self.content.clone();

        result
    }
}

impl<T: DrawTarget> PartialEq for Component<T> {
    fn eq(&self, other: &Component<T>) -> bool {
        self.id() == other.id()
    }
}

mod test {
    extern crate alloc;
    use alloc::{boxed::Box, vec};
    use embedded_graphics::{
        prelude::{Point, Primitive},
        primitives::{Circle, PrimitiveStyle},
    };

    #[test]
    fn test_find_comp() {
        use super::*;
        use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};

        let mut comp0: Component<MockDisplay<Rgb565>> = Component::new();

        let mut comp1: Component<MockDisplay<Rgb565>> = Component::new();
        comp1.set_content(Some(vec![Element::new(Box::new(
            Circle::new(Point::zero(), 0).into_styled(PrimitiveStyle::default()),
        ))]));

        let mut comp2: Component<MockDisplay<Rgb565>> = Component::new();
        // We don't need to call `clone_with_new_id' here because this is our first use of `comp1'.
        comp2.add_child(comp1.clone());

        let mut comp3: Component<MockDisplay<Rgb565>> = Component::new();
        // Create a new ID to avoid conflicting IDs.
        comp3.add_child(comp1.clone_with_new_id());

        comp0.add_child(comp2).add_child(comp3);

        assert!(comp0.find_comp_by_id(comp1.id()) == Some(&comp1));
    }
}
