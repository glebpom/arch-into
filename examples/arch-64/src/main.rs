use arch_into::{ArchInto, ArchFrom};

fn main() {
    let a: u64 = 42;
    let b: usize = a.arch_into();
    let _c = u64::arch_from(b);
}
