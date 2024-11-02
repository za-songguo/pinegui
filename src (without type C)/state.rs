extern crate alloc;

use crate::{component::Component, elememt::Element, id::ItemID, ui::Ui};
use alloc::vec::Vec;
use embedded_graphics::draw_target::DrawTarget;

#[derive(Clone, PartialEq)]
pub struct State<V, T: DrawTarget> {
    value: V,
    ui: Ui<T>,
    /// A reference to the component that uses the value of this state.
    related_comps: Vec<Component<T>>,
}

impl<V, T: DrawTarget> State<V, T> {
    /// Create a new state with the given value.
    pub fn new(value: V, ui: Ui<T>) -> Self {
        Self {
            value,
            ui,
            related_comps: Vec::new(),
        }
    }

    /// Set the value of the state and return all related components.
    pub fn set(&mut self, value_fn: impl Fn(&V) -> V) {
        self.value = value_fn(&self.value);

        let mut new_render_list = Vec::new();

        // Find the differences between the old and new components and create a new render list with only the differences that are needed for rendering.
        for comp in &self.related_comps {
            let former_comp = self.ui.child_comp.find_comp_by_id(comp.id());

            // If we can find the former component, compare it with the new component and add the differences to the result.
            if let Some(former_comp) = former_comp {
                new_render_list.extend(diff(former_comp.content.clone(), comp.content.clone()))
            }
        }

        self.ui.render_list.extend(new_render_list);
    }

    /// Get the value of the state and bind the component to the state to trigger re-rendering when the state value changes.
    pub fn value(&mut self, component: Component<T>) -> &V {
        // Avoid adding dupicate components.
        if !self.related_comps.contains(&component) {
            self.related_comps.push(component);
        }
        &self.value
    }
}

fn diff<T: DrawTarget>(a: Option<Vec<Element<T>>>, b: Option<Vec<Element<T>>>) -> Vec<Element<T>> {
    // Some + Some => find differences between them
    // Some + None or None + Some => return the item which is Some
    // None + None => None
    match (a.clone(), b.clone()) {
        (Some(a), Some(b)) => {
            let mut result = Vec::new();

            for i in &a {
                if !b.contains(i) {
                    result.push(i.clone());
                }
            }

            for i in &b {
                if !a.contains(i) {
                    result.push(i.clone());
                }
            }

            result
        }
        (None, None) => Vec::new(),
        _ => a.or(b).unwrap(),
    }
}
