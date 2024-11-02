#![no_std]

extern crate alloc;

use core::fmt;

use alloc::boxed::Box;

use embedded_graphics::prelude::Point;

pub mod keyboard;
pub mod mouse;

/// A trait to get the position of an event that has happened.
pub trait EventPosition: dyn_clone::DynClone {
    /// If the method returns `Some`, this means that the event is associated with a position, such as a click event.
    /// Otherwise, it means that the event is not associated with a position (global event), such as a key event.
    ///
    /// An event with position is sent to the element on which the event occurred.
    /// An global event is sent to all elements.
    fn position(&self) -> Option<Point>;
}

dyn_clone::clone_trait_object!(EventPosition);

impl PartialEq for dyn EventPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position() == other.position()
    }
}

impl Eq for dyn EventPosition {}

impl fmt::Debug for dyn EventPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.position().fmt(f)
    }
}

/// A type that represents an event.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// Clicked with the mouse or touched something on a touch screen.
    Click(mouse::ClickEvent),
    /// Mouse button released or hand released something on a touch screen.
    Release(mouse::ClickEvent),
    /// The mouse pointer or touch moved to a new place.
    PointerMoved(mouse::PointerMoveEvent),
    /// A scroll occurred on the mouse wheel, or a swipe event occurred on the touch screen.
    Scroll(mouse::ScrollEvent),
    /// A key on the keyboard was pressed.
    KeyDown(keyboard::KeyCode),
    /// A key on the keyboard was released.
    KeyUp(keyboard::KeyCode),
    /// A key on the keyboard was auto-repeated.
    ///
    /// When you hold down a key, most keyboards and operating systems automatically start generating key events continuously after a short delay.
    KeyRepeated(keyboard::KeyCode),
    /// A variant that contains a user-specified custom event.
    Custom(Box<dyn EventPosition>),
}

impl EventPosition for Event {
    fn position(&self) -> Option<Point> {
        match self {
            Self::Click(event) | Self::Release(event) => event.position(),
            Self::PointerMoved(event) => event.position(),
            Self::Scroll(event) => event.position(),

            Self::KeyUp(_) | Self::KeyDown(_) | Self::KeyRepeated(_) => None,

            Self::Custom(event) => event.position(),
        }
    }
}
