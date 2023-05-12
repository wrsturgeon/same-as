//! Type equality in stable Rust.
//! # Use
//! A toy example that demonstrates the basics:
//! ```rust
//! use same_as::SameAs;
//! trait Eat<T> { fn eat<U: SameAs<T>>(_: U); } // Please don't actually write this
//! struct MrCreosote;
//! impl Eat<u8> for MrCreosote { fn eat<U: SameAs<u8>>(_: U) {} }
//! MrCreosote::eat(0_u8); // wafer-thin type
//! ```
//! This won't compile:
//! ```compile_fail
//! // ...
//! # use same_as::SameAs;
//! # trait Eat<T> { fn eat<U: SameAs<T>>(_: U); }
//! struct MrCreosote;
//! impl Eat<u8> for MrCreosote { fn eat<U: SameAs<u8>>(_: U) {} }
//! MrCreosote::eat(0_u16); // kaboom
//! ```
//! # But why is type equality necessary?
//! Sometimes you need it where Rust can't leverage it now, e.g. defining a Haskell-style monad in Rust:
//! ```rust
//! # use same_as::SameAs;
//! pub trait Monad<A>: SameAs<Self::Constructor<A>> { // <-- Enforces that e.g. for `Maybe<A>`, `Self::Constructor` is effectively just the type constructor `Maybe`.
//!     type Constructor<B>: Monad<B>; // In this `impl`, `Self` is really `Self<A>`, but we want to make `Self<B>` below.
//!     fn bind<B, F: Fn(A) -> B>(self, f: F) -> Self::Constructor<B>;
//! }
//! ```
//! So this would work:
//! ```rust
//! # use same_as::SameAs;
//! # pub trait Monad<A>: SameAs<Self::Constructor<A>> { type Constructor<B>; }
//! pub enum Maybe<A> { Nothing, Just(A) }
//! impl<A> Monad<A> for Maybe<A> { type Constructor<B> = Maybe<B>; }
//! ```
//! but we can prove that this won't, and so we can safely simulate type constructors in Rust:
//! ```compile_fail
//! # use same_as::SameAs;
//! # pub trait Monad<A>: SameAs<Self::Constructor<A>> { type Constructor<B>; }
//! pub enum Maybe<A> { Nothing, Just(A) } // deception!  vvvvvv
//! impl<A> Monad<A> for Maybe<A> { type Constructor<B> = Option<B>; }
//! ```

#![no_std]
#![deny(warnings)]
#![warn(
    clippy::all,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    clippy::cargo,
    missing_docs,
    rustdoc::all
)]
#![allow(clippy::blanket_clippy_restriction_lints)]

/// Sealed trait so end-users can't override the one implementation.
mod sealed {
    /// Type equality.
    pub trait SameAs<T> {}
    impl<T> SameAs<T> for T {}
}

/// Type equality.
pub trait SameAs<T>: sealed::SameAs<T> {}
impl<T> SameAs<T> for T {}
