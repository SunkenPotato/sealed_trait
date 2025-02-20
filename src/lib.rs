//! # `sealed_trait`
//! A utility for generating sealed traits in Rust.
//! Inspired by Java's `sealed class` syntax.
//!
//! ## Usage
//! You can create a sealed trait by using the `sealed_trait` macro:
//! ```
//! sealed_trait! {
//!     pub sealed trait TestTrait permits i32 {
//!         fn print_me(self);
//!     }
//!
//!     impl TestTrait for i32 {
//!         fn print_me(self) {
//!             println!("{self}")
//!         }
//!     }
//! }
//! ```
//! You can also add supertraits to your traits, but they have to be inside square brackets and separated by **commas**, not `+`:
//! ```
//! sealed_trait! {
//!     pub sealed trait TestTrait: [Sized, Into<i32>] permits i32 {
//!         ...
//!     }
//! }
//! ```

#![no_std]
mod macros;
pub use paste;

#[cfg(test)]
#[allow(unused)]
mod tests {
    super::sealed_trait! {
        pub sealed trait TestTrait permits i32 => {
            fn print_me(self);
        }
    }

    super::sealed_trait! {
        pub sealed trait TraitWithBounds: [Sized, Into<i32>] permits i32 => {
            fn print_me(self);
        }
    }
}
