// /// Platform-specific timer implementation.
// pub trait Timer<T, const NOM: u32 = 1, const DENOM: u32 = 1000> {
//     /// Executes the closure every `NOM/DENOM` seconds, repeating until the closure returns `false`.
//     ///
//     /// Reference implementation:
//     /// ```rust
//     /// impl Timer<u32, 1, 1000> for MyTimer {
//     ///     fn set_interval(
//     ///         &self,
//     ///         interval: fugit::Duration<u32, 1, 1000>,
//     ///         closure: impl Fn(&Self) -> bool,
//     ///     ) {
//     ///         loop {
//     ///             // If the closure returns false, stop the loop.
//     ///             if !(closure)(&self) {
//     ///                 break;
//     ///             }
//     ///             self.delay(interval);
//     ///         }
//     ///     }
//     /// }
//     /// ```
//     fn set_interval(
//         &self,
//         interval: fugit::Duration<T, NOM, DENOM>,
//         closure: impl Fn(&Self) -> bool,
//     );
// }
