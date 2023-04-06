use arch_into::{ArchInto, ArchFrom};

fn main() {
    let a: u32 = 23;
    let b: usize = a.arch_into();
    let _c = u32::arch_from(b);
}
