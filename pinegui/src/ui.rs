extern crate alloc;

use core::fmt;

use alloc::vec::Vec;

use dyn_ord::DynEq;
use embedded_graphics::{geometry, primitives};
use pinegui_event::{Event, EventPosition};

use crate::{component::Component, elememt::Element, node::Node, shared::Shared};

pub struct Ui {
    bounding_box: primitives::Rectangle,
    pub node_tree: Option<Shared<Node>>,
}

impl Ui {
    pub const fn new(bounding_box: primitives::Rectangle) -> Self {
        Self {
            bounding_box,
            node_tree: None,
        }
    }

    pub fn set_node_tree(&mut self, node_tree: Shared<Node>) {
        self.node_tree = Some(node_tree);
    }

    pub fn dispatch_event(&self, event: Event) {
        if let Some(node_tree) = &self.node_tree {
            let elements = node_tree.expand();

            // Dispatch the event to the element if the event is triggered on the element.
            // If it is a global event, dispatch it to all elements.
            let elements: Vec<_> = match event.position() {
                Some(position) => elements
                    .into_iter()
                    .filter(|e| e.content.bounding_box().contains(position))
                    .collect(),
                None => elements,
            };
            elements
                .iter()
                .for_each(|e| (e.event_callback)(event.clone()))
        }
    }
}

impl fmt::Debug for Ui {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Ui")
            .field("bounding_box", &self.bounding_box)
            .field("node_tree", &self.node_tree)
            .finish()
    }
}

impl geometry::Dimensions for Ui {
    fn bounding_box(&self) -> primitives::Rectangle {
        self.bounding_box
    }
}

impl Clone for Ui {
    fn clone(&self) -> Self {
        Self {
            bounding_box: self.bounding_box,
            node_tree: self.node_tree.clone(),
        }
    }
}

impl PartialEq for Ui
where
    dyn Component: DynEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bounding_box == other.bounding_box && self.node_tree == other.node_tree
    }
}

impl Eq for Ui where dyn Component: DynEq {}

// TODO move to a new place
/// Find the differences between two vectors and return the differences as a new vector.
pub fn find_differences(a: Vec<Element>, b: Vec<Element>) -> Vec<Element> {
    let mut result = Vec::with_capacity(a.len() + b.len());

    for element in a.clone() {
        if !b.contains(&element) {
            result.push(element);
        }
    }

    for element in b {
        if !a.contains(&element) {
            result.push(element);
        }
    }

    result
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     extern crate alloc;

//     use alloc::{boxed::Box, vec};
//     use geometry::Point;
//     use primitives::{Circle, Primitive};

//     // test find differences
//     #[test]
//     fn test_find_differences() {
//         let cb: Shared<Box<dyn Fn(Event) + Send>> = Shared::new(Box::new(|_| {}));

//         let a: Vec<Element> = vec![
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 10).into_styled(Default::default())),
//                 cb.clone(),
//             ),
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 20).into_styled(Default::default())),
//                 cb.clone(),
//             ),
//         ];

//         let b: Vec<Element> = vec![
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 10).into_styled(Default::default())),
//                 cb.clone(),
//             ),
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 30).into_styled(Default::default())),
//                 cb.clone(),
//             ),
//         ];

//         let expected: Vec<Element> = vec![
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 20).into_styled(Default::default())),
//                 cb.clone(),
//             ),
//             Element::new(
//                 Box::new(Circle::new(Point::zero(), 30).into_styled(Default::default())),
//                 cb,
//             ),
//         ];

//         assert!(find_differences(a, b) == expected);
//     }
// }
