extern crate alloc;

use core::fmt;

use alloc::vec::Vec;
use embedded_graphics::prelude::DrawTarget;

use crate::{component::Component, node::Node, shared::Shared};

pub struct State<V, T: DrawTarget> {
    value: V,
    related_comps: Vec<Shared<Node<T>>>,
}

impl<V: Clone, T: DrawTarget> Clone for State<V, T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            related_comps: self.related_comps.clone(),
        }
    }
}

impl<V: fmt::Debug, T: DrawTarget> fmt::Debug for State<V, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("State")
            .field("value", &self.value)
            .field("related_comps", &self.related_comps)
            .finish()
    }
}

impl<V, T: DrawTarget> State<V, T> {
    /// Create a new state with the given value.
    pub fn new(value: V) -> Shared<Self> {
        Shared::new(Self {
            value,
            related_comps: Vec::new(),
        })
    }

    /// Set the value of the state and re-render all related components.
    pub fn set(&mut self, value_fn: impl Fn(&V) -> V) {
        self.value = value_fn(&self.value);
        self.force_rerender_related_comps();
    }

    /// If the new value is different from the old value, set the value of the state and re-render all related components. Otherwise nothing is done.
    pub fn set_eq(&mut self, value_fn: impl Fn(&V) -> V)
    where
        V: PartialEq,
    {
        let new_value = value_fn(&self.value);
        if !(self.value == new_value) {
            self.value = new_value;
            self.force_rerender_related_comps();
        }
    }

    /// Get the value of the state and bind the component to the state to trigger re-rendering when the state value changes.
    pub fn value(&mut self, component: Shared<Node<T>>) -> &V
    where
        dyn Component<T>: PartialEq,
    {
        // Avoid adding dupicate components.
        if !self.related_comps.contains(&component.clone()) {
            self.related_comps.push(component);
        }
        &self.value
    }

    /// Force all related components to be marked as needing to be re-rendered.
    ///
    /// In most cases, you won't need to call this method manually.
    pub fn force_rerender_related_comps(&self) {
        self.related_comps
            .iter()
            .for_each(|c| c.borrow_mut().need_rerender = true); // Because the node is under a `Shared`, the change will be reflected to the corresponding `Node` in `Ui`.
    }
}

impl<V: PartialEq, T: DrawTarget> PartialEq for State<V, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Eq, T: DrawTarget> Eq for State<V, T> {}

impl<V: PartialOrd, T: DrawTarget> PartialOrd for State<V, T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<V: Ord, T: DrawTarget> Ord for State<V, T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
