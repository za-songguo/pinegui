extern crate alloc;

use core::fmt;

use alloc::vec::Vec;
use dyn_ord::DynEq;

use crate::{component::Component, node::Node, shared::Shared};

pub struct State<V> {
    value: V,
    related_comps: Vec<Shared<Node>>,
}

impl<V: Clone> Clone for State<V> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            related_comps: self.related_comps.clone(),
        }
    }
}

impl<V: fmt::Debug> fmt::Debug for State<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("State")
            .field("value", &self.value)
            .field("related_comps", &self.related_comps)
            .finish()
    }
}

impl<V> State<V> {
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
    pub fn value(&mut self, component: Shared<Node>) -> &V
    where
        dyn Component: DynEq,
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
            .for_each(|c| c.borrow_mut().need_rerender = true); // Because the node is under a `Shared`he change will be reflected to the corresponding `Node` in `Ui`.
    }
}

impl<V: PartialEq> PartialEq for State<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: Eq> Eq for State<V> {}

impl<V: PartialOrd> PartialOrd for State<V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<V: Ord> Ord for State<V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
