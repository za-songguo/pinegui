use embedded_graphics::prelude::Point;

use crate::EventPosition;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickEvent {
    Left(Point),
    Right(Point),
    Middle(Point),
    LeftLong(Point),
    RightLong(Point),
    MiddleLong(Point),
}

impl EventPosition for ClickEvent {
    fn position(&self) -> Option<Point> {
        let point = match self {
            ClickEvent::Left(p) => *p,
            ClickEvent::Right(p) => *p,
            ClickEvent::Middle(p) => *p,
            ClickEvent::LeftLong(p) => *p,
            ClickEvent::RightLong(p) => *p,
            ClickEvent::MiddleLong(p) => *p,
        };
        Some(point)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollEvent {
    /// The current pointer position.
    pub position: Point,
    /// The amount of scrolling.
    ///
    /// Positive values (`x` or `y`) represent the right or downward direction and negative values represent the left or upward direction.
    pub delta: Point,
}

impl EventPosition for ScrollEvent {
    fn position(&self) -> Option<Point> {
        Some(self.position)
    }
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PointerMoveEvent {
    /// The current pointer position.
    pub position: Point,
}

impl EventPosition for PointerMoveEvent {
    fn position(&self) -> Option<Point> {
        Some(self.position)
    }
}
