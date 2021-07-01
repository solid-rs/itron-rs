//! Provides macros that allow downstream crates to examine the choice of
//! the target kernel.
//!
//! This module's macros whose names start with `tt_` follows `tt-call`'s token
//! tree calling convention.

include!(concat!(env!("OUT_DIR"), "/macros.rs"));

// Make `tt_call` available to the following macros' expansion
#[doc(hidden)]
pub use tt_call;

/// Expand to the current kernel's name (e.g., `"asp3"`).
///
/// # Examples
///
/// ```rust
/// println!("We are running on {}", itron::macros::kernel!());
/// ```
///
/// ```rust,compile_fail
/// compile_error!(concat!("kernel `", itron::macros::kernel!(), "` is not supported"));
/// ```
pub macro kernel() {
    tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] }
}

/// Expand to the arm corresponding to the current kernel.
///
/// # Example
///
/// ```rust
/// itron::macros::match_kernel! {
///     "asp3" | "solid_asp3" => { fn say() { println!("We are running on TOPPERS/ASP3, yay!"); } }
///     "nonexistent_kernel" => { call_nonexistent_function(); }
///     _ => { fn say() { println!("This kernel looks like something new!"); } }
/// }
/// say();
/// ```
///
/// The arms don't create local scopes, and unselected arms are eliminated
/// during an early stage of compilation. Compare to the following example:
///
/// ```rust,compile_fail
/// match itron::macros::kernel!() {
///     "asp3" | "solid_asp3" => { fn say() { println!("We are running on TOPPERS/ASP3, yay!"); } }
///     "nonexistent_kernel" => { call_nonexistent_function(); }
///         // ERROR: `call_nonexistent_function` is undefined
///     _ => { fn say() { println!("This kernel looks like something new!"); } }
/// }
/// say(); // ERROR: Each arm's `say` is not accessible from here
/// ```
///
pub macro match_kernel {
    (
        _ => { $($wildcard:tt)* }
    ) => { $($wildcard)* },
    (
        _ => { $($wildcard:tt)* }
        $($rest:tt)*
    ) => {
        compile_error!("anything that follows `_ => { ... }` never match")
    },
    (
        $( $kernel:tt )|+ => { $($tt:tt)* }
        $($rest:tt)*
    ) => {
        tt_call::tt_if! {
            condition = [{ $crate::macros::tt_is_kernel }]
            input = [{ $( $kernel )|+ }]
            true = [{ $($tt)* }]
            false = [{
                match_kernel! { $($rest)* }
            }]
        }
    },
}
