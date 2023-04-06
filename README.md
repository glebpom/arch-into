# arch-into

This crates provides simplified conversion for usize/isize types, 
for the list of supported pointer width. 

Typically, you, when you want to convers usize, to u64/u32 you have few options:

- Use `as` keyword. This approach may lead to incorrect results
- Use `try_from` with `unwrap`/`expect`. When you target only 64-bits architectures this is fin, but produces a lot of boilerplate code
- Use `try_from` and return error. This approach hides misbehavior of your code. 

This crates provides two features: `arch-32` and `arch-64`. It defines the minimum supported architecture. If you try to compile fo unsupported architecture, comiplation will fail with error.\
Since minimum supported pointer width is defined, we can define safe conversions for types with specific size.

## Usage

```rust
use arch_into::{ArchInto, ArchFrom};

fn main() {
    let a: u64 = 23;
    let b: usize = a.arch_into();
    let _c = u64::arch_from(b);
}
```
