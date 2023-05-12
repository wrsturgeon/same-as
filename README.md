# Type equality in stable Rust.

## Use
A toy example that demonstrates the basics:
```rust
use same_as::SameAs;
trait Eat<T> { fn eat<U: SameAs<T>>(_: U); } // Please don't actually write this
struct MrCreosote;
impl Eat<u8> for MrCreosote { fn eat<U: SameAs<u8>>(_: U) {} }
MrCreosote::eat(0_u8); // wafer-thin type
```
This won't compile:
```rust
// ...
struct MrCreosote;
impl Eat<u8> for MrCreosote { fn eat<U: SameAs<u8>>(_: U) {} }
MrCreosote::eat(0_u16); // kaboom
```
## But why is type equality necessary?
Sometimes you need it where Rust can't leverage it now, e.g. defining a Haskell-style monad in Rust:
```rust
pub trait Monad<A>: SameAs<Self::Constructor<A>> { // <-- Enforces that e.g. for `Maybe<A>`, `Self::Constructor` is effectively just the type constructor `Maybe`.
    type Constructor<B>: Monad<B>; // In this `impl`, `Self` is really `Self<A>`, but we want to make `Self<B>` below.
    fn bind<B, F: Fn(A) -> B>(self, f: F) -> Self::Constructor<B>;
}
```
So this would work:
```rust
pub enum Maybe<A> { Nothing, Just(A) }
impl<A> Monad<A> for Maybe<A> { type Constructor<B> = Maybe<B>; }
```
but we can prove that this won't, and so we can safely simulate type constructors in Rust:
```rust
pub enum Maybe<A> { Nothing, Just(A) } // deception!  vvvvvv
impl<A> Monad<A> for Maybe<A> { type Constructor<B> = Option<B>; }
```

