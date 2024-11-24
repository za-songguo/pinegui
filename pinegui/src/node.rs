extern crate alloc;

use core::fmt;

use crate::{component::Component, elememt::Element, shared::Shared};
use alloc::{boxed::Box, vec::Vec};
use dyn_ord::DynEq;

pub struct Node {
    pub need_rerender: bool,
    pub component: Option<Shared<Box<dyn Component>>>,
    pub children: Vec<Shared<Node>>,
    // TODO parent (Weak)
    pub content: Vec<Element>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            need_rerender: true,
            component: None,
            children: Vec::new(),
            content: Vec::new(),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            need_rerender: self.need_rerender,
            component: self.component.clone(),
            children: self.children.clone(),
            content: self.content.clone(),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Node")
            .field("need_rerender", &self.need_rerender)
            .field("component", &self.component)
            .field("children", &self.children)
            .field("content", &self.content)
            .finish()
    }
}

impl PartialEq for Node
where
    dyn Component: DynEq,
{
    fn eq(&self, other: &Self) -> bool {
        // XXX: Will this implmentation cause problems?
        self.dyn_eq(other.as_any().downcast_ref::<&dyn DynEq>().unwrap())
            && self.content == other.content
            && self.children == other.children
    }
}

impl Eq for Node where dyn Component: DynEq {}

impl Node {
    pub fn new() -> Shared<Self> {
        Shared::new(Self::default())
    }

    pub fn set_component(&mut self, component: Shared<Box<dyn Component>>) -> &mut Self {
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

    pub fn set_content(&mut self, content: Vec<Element>) -> &mut Self {
        self.content = content;
        self
    }

    pub fn set_need_rerender(&mut self, need_rerender: bool) -> &mut Self {
        self.need_rerender = need_rerender;
        self
    }

    // XXX expand: 展开全部元素 iter: 展开全部节点
    pub fn expand(&self) -> Vec<Element> {
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
