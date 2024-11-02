extern crate alloc;

use core::fmt;

use crate::{component::Component, elememt::Element, shared::Shared};
use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::draw_target::DrawTarget;

pub struct Node<T: DrawTarget> {
    pub need_rerender: bool,
    pub component: Option<Shared<Box<dyn Component<T>>>>,
    pub children: Vec<Shared<Node<T>>>,
    pub content: Vec<Element<T>>,
}

impl<T: DrawTarget> Default for Node<T> {
    fn default() -> Self {
        Self {
            need_rerender: true,
            component: None,
            children: Vec::new(),
            content: Vec::new(),
        }
    }
}

impl<T: DrawTarget> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            need_rerender: self.need_rerender,
            component: self.component.clone(),
            children: self.children.clone(),
            content: self.content.clone(),
        }
    }
}

impl<T: DrawTarget> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("need_rerender", &self.need_rerender)
            .field("component", &self.component)
            .field("children", &self.children)
            .field("content", &self.content)
            .finish()
    }
}

impl<T: DrawTarget> PartialEq for Node<T>
where
    dyn Component<T>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        // XXX: Will this implmentation cause problems?
        self.component == other.component
            && self.content == other.content
            && self.children == other.children
    }
}

impl<T: DrawTarget> Eq for Node<T> where dyn Component<T>: PartialEq {}

impl<T: DrawTarget> Node<T> {
    pub fn new() -> Shared<Self> {
        Shared::new(Self::default())
    }

    pub fn set_component(&mut self, component: Shared<Box<dyn Component<T>>>) -> &mut Self {
        self.component = Some(component);
        self
    }

    pub fn add_child(&mut self, child: Shared<Self>) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn set_children(&mut self, children: Vec<Shared<Self>>) -> &mut Self {
        self.children = children;
        self
    }

    pub fn set_content(&mut self, content: Vec<Element<T>>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn set_need_rerender(&mut self, need_rerender: bool) -> &mut Self {
        self.need_rerender = need_rerender;
        self
    }

    // XXX expand: 展开全部元素 iter: 展开全部节点
    pub fn expand(&self) -> Vec<Element<T>> {
        let mut result = Vec::new();
        result.extend(self.content.clone());

        self.children
            .iter()
            .for_each(|child| result.extend(child.expand()));

        result
    }

    pub fn expand_node(node: Shared<Self>) -> Vec<Shared<Self>> {
        let mut result = Vec::new();
        result.push(node.clone());

        node.children
            .iter()
            .for_each(|child| result.extend(Node::expand_node(child.clone())));

        result
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::Rgb565,
        prelude::{Point, Primitive},
        primitives::Circle,
    };

    use super::*;

    #[test]
    fn test_expand_nodes() {
        let node = Node::<MockDisplay<Rgb565>>::new();

        node.borrow_mut().add_child(Node::new());
        node.borrow_mut().add_child(
            Node::new()
                .borrow_mut()
                .add_child(Node::new().borrow_mut().add_child(Node::new()).into())
                .into(),
        );

        let mut count = 0;

        for _ in Node::expand_node(node) {
            count += 1;
        }
        assert_eq!(count, 5);
    }

    #[test]
    fn test_expand() {
        let node = Node::<MockDisplay<Rgb565>>::new();

        node.borrow_mut().set_content(vec![
            Element::new(
                Box::new(Circle::new(Point::zero(), 0).into_styled(Default::default())),
                Shared::new(Box::new(|_| {})),
            ),
            Element::new(
                Box::new(Circle::new(Point::zero(), 1).into_styled(Default::default())),
                Shared::new(Box::new(|_| {})),
            ),
        ]);

        node.borrow_mut().add_child(Node::new());
        node.borrow_mut().add_child(
            Node::new()
                .borrow_mut()
                .add_child(Node::new().borrow_mut().add_child(Node::new()).into())
                .into(),
        );

        let mut count = 0;

        for _ in node.clone().expand() {
            count += 1;
        }
        assert_eq!(count, 2);
    }
}
