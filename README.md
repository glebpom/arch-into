# arch-into

This crate provides simplified conversions between `usize`/`isize` types, and types with constant size, depending on supported architectures.

Typically, when you want to convert `usize` to `u64` (or `u32`) you have few options:

- Use `as` keyword. This approach may lead to incorrect results
- Use `try_from` with `unwrap`/`expect`. When you target only 64-bits architectures this is fine, but it produces a lot of boilerplate
- Use `try_from` and return error. This approach hides misbehavior of your code. 

This crates provides two features: `arch-32` and `arch-64`. It defines the supported pointer width. If you try to compile for unsupported architecture, compilation will fail with the error.

Since minimum supported pointer width is defined, we can use safe conversions for types with specific size.

## Usage

```rust
use arch_into::{ArchInto, ArchFrom};

fn main() {
    let a: u64 = 23;
    let b: usize = a.arch_into();
    let _c = u64::arch_from(b);
}
```
