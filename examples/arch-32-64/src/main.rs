use arch_into::ArchFrom;

fn main() {
    let b: usize = 42;
    let _c = u64::arch_from(b);
}
