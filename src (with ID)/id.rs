use core::sync::atomic::{AtomicUsize, Ordering};

use crate::component::Component;
use embedded_graphics::draw_target::DrawTarget;
use pinegui_event::Event;

pub static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

pub trait ItemID {
    fn id(&self) -> usize;

    fn new_id() -> usize {
        NEXT_ID.fetch_add(1, Ordering::Relaxed)
    }
}

impl<T: DrawTarget, C: FnOnce(Event) + Clone> ItemID for Component<T, C> {
    fn id(&self) -> usize {
        self.id
    }
}
