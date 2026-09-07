# arch-into

This crate provides simplified conversions between `usize`/`isize` types, and types with constant size, depending on supported architectures.

Typically, when you want to convert `usize` to `u64` (or `u32`) you have few options:

- Use `as` keyword. This approach may lead to incorrect results
- Use `try_from` with `unwrap`/`expect`. When you target only 64-bits architectures this is fine, but it produces a lot of boilerplate
- Use `try_from` and return error. This approach hides misbehavior of your code. 

This crate supports both 32-bit and 64-bit targets by default. The `no-arch-32` and `no-arch-64` features exclude the corresponding pointer width and enable additional conversions. Compiling for an excluded pointer width fails with an error.

Since unsupported pointer width is defined, we can use safe conversions for types with specific size.

## Usage

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
arch-into = "0.0.1-alpha.5"
```

This example works with the default features on both 32-bit and 64-bit targets:

```rust
use arch_into::{ArchFrom, ArchInto};

fn main() {
    let a: u32 = 23;
    let b: usize = a.arch_into();
    let c = u64::arch_from(b);
    assert_eq!(c, 23);
}
```

For conversions from `u64` to `usize` or from `i64` to `isize`, restrict the crate to 64-bit targets:

```toml
[dependencies]
arch-into = { version = "0.0.1-alpha.5", features = ["no-arch-32"] }
```

Conversely, `no-arch-64` restricts the crate to 32-bit targets and enables conversions from `usize` to `u32` and from `isize` to `i32`. Enable only the feature that matches your supported targets; enabling both excludes both pointer widths.
