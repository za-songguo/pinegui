extern crate alloc;

use alloc::boxed::Box;

/// Platform-specific timer implementation.
pub trait Timer {
    /// Set an interval.
    ///
    /// If the closure returns true, the interval will continue repeating.
    ///
    /// Reference implementation (or use interrupts):
    /// ```rust
    /// pub fn set_interval(interval: fugit::Duration<u32, 1, 1000>, closure: Box<dyn Fn() -> bool>) {
    ///     loop {
    ///         // If the closure returns false, stop the loop.
    ///         if !(closure)() {
    ///             break;
    ///         }
    ///         delay(self.interval);
    ///     }
    /// }
    /// ```
    fn set_interval(interval: fugit::Duration<u32, 1, 1000>, closure: Box<dyn Fn() -> bool>);
}
