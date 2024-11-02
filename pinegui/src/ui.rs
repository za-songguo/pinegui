extern crate alloc;

use core::{any::Any, fmt};

use alloc::vec::Vec;

use embedded_graphics::{draw_target::DrawTarget, geometry, primitives};
use pinegui_event::{Event, EventPosition};
use pinegui_renderer::render::Render;

use crate::{component::Component, elememt::Element, node::Node, shared::Shared};

pub struct Ui<T: DrawTarget> {
    bounding_box: primitives::Rectangle,
    pub node_tree: Option<Shared<Node<T>>>,
}

impl<T: DrawTarget> Ui<T> {
    pub const fn new(bounding_box: primitives::Rectangle) -> Self {
        Self {
            bounding_box,
            node_tree: None,
        }
    }

    pub fn set_node_tree(&mut self, node_tree: Shared<Node<T>>) {
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

impl<T: DrawTarget> fmt::Debug for Ui<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Ui")
            .field("bounding_box", &self.bounding_box)
            .field("node_tree", &self.node_tree)
            .finish()
    }
}

impl<T: DrawTarget> geometry::Dimensions for Ui<T> {
    fn bounding_box(&self) -> primitives::Rectangle {
        self.bounding_box
    }
}

impl<T: DrawTarget> Clone for Ui<T> {
    fn clone(&self) -> Self {
        Self {
            bounding_box: self.bounding_box,
            node_tree: self.node_tree.clone(),
        }
    }
}

impl<T: DrawTarget + 'static> PartialEq for Ui<T>
where
    dyn Component<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.bounding_box == other.bounding_box && self.node_tree == other.node_tree
    }
}

impl<T: DrawTarget + 'static> Eq for Ui<T> where dyn Component<T>: PartialEq {}

impl<T: DrawTarget + 'static> Render<T> for Ui<T>
where
    dyn Component<T>: PartialEq,
{
    fn render(&self, target: &mut T) -> Result<(), <T as DrawTarget>::Error> {
        // Loop through the node tree and re-render any nodes marked as needing re-rendering.
        if let Some(node_tree) = &self.node_tree {
            let old_elements = node_tree.expand();

            // Find all nodes with `need_rerender = true`, call `view`, expand, find different elements, and render.
            let new_elements = Node::expand_node(node_tree.clone())
                .into_iter()
                .filter(|node| node.need_rerender)
                .flat_map(|node| {
                    let component = node
                        .component
                        .clone()
                        .expect("You should call `set_component` on `Node` before rendering.");

                    let new_node = component.borrow_mut().view(self);

                    new_node.borrow_mut().set_need_rerender(false);
                    new_node.borrow_mut().set_component(component);

                    *node.borrow_mut() = (*new_node).clone();
                    node.expand()
                })
                .collect::<Vec<_>>();

            let differences = find_differences(old_elements, new_elements);
            for element in differences {
                element.content.render(target)?;
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn is_eq(&self, other: &dyn Render<T>) -> bool {
        if let Some(other_concrete) = other.as_any().downcast_ref::<Self>() {
            self == other_concrete
        } else {
            false
        }
    }
}

// TODO move to a new place
/// Find the differences between two vectors and return the differences as a new vector.
pub fn find_differences<T: DrawTarget>(a: Vec<Element<T>>, b: Vec<Element<T>>) -> Vec<Element<T>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    extern crate alloc;

    use alloc::{boxed::Box, vec};
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb565};
    use geometry::Point;
    use primitives::{Circle, Primitive};

    // test find differences
    #[test]
    fn test_find_differences() {
        let cb: Shared<Box<dyn Fn(Event) + Send>> = Shared::new(Box::new(|_| {}));

        let a: Vec<Element<MockDisplay<Rgb565>>> = vec![
            Element::new(
                Box::new(Circle::new(Point::zero(), 10).into_styled(Default::default())),
                cb.clone(),
            ),
            Element::new(
                Box::new(Circle::new(Point::zero(), 20).into_styled(Default::default())),
                cb.clone(),
            ),
        ];

        let b: Vec<Element<MockDisplay<Rgb565>>> = vec![
            Element::new(
                Box::new(Circle::new(Point::zero(), 10).into_styled(Default::default())),
                cb.clone(),
            ),
            Element::new(
                Box::new(Circle::new(Point::zero(), 30).into_styled(Default::default())),
                cb.clone(),
            ),
        ];

        let expected: Vec<Element<MockDisplay<Rgb565>>> = vec![
            Element::new(
                Box::new(Circle::new(Point::zero(), 20).into_styled(Default::default())),
                cb.clone(),
            ),
            Element::new(
                Box::new(Circle::new(Point::zero(), 30).into_styled(Default::default())),
                cb,
            ),
        ];

        assert!(find_differences(a, b) == expected);
    }
}
