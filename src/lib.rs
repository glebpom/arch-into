#[cfg(all(target_pointer_width = "32", feature = "arch-64"))]
compile_error!("arch-into is configured for 64-bits, but your target architecture is 32-bits");

#[cfg(all(target_pointer_width = "16", feature = "arch-64"))]
compile_error!("arch-into is configured for 64-bits, but your target architecture is 16-bits");

#[cfg(all(target_pointer_width = "8", feature = "arch-64"))]
compile_error!("arch-into is configured for 64-bits, but your target architecture is 8-bits");

#[cfg(all(target_pointer_width = "16", feature = "arch-32"))]
compile_error!("arch-into is configured for 32-bits, but your target architecture is 16-bits");

#[cfg(all(target_pointer_width = "8", feature = "arch-32"))]
compile_error!("arch-into is configured for 32-bits, but your target architecture is 8-bits");

pub trait ArchInto<T>: Sized {
    fn arch_into(self) -> T;
}

pub trait ArchFrom<T>: Sized {
    fn arch_from(value: T) -> Self;
}

#[allow(unused_macros)]
macro_rules! define_conversion {
    ($from:ty, $to:ty) => {
        impl ArchInto<$to> for $from {
            fn arch_into(self) -> $to {
                self.try_into().unwrap()
            }
        }
        impl ArchFrom<$from> for $to {
            fn arch_from(value: $from) -> Self {
                value.try_into().unwrap()
            }
        }
    };
}

#[cfg(feature = "arch-64")]
mod arch64 {
    use super::*;

    define_conversion!(u64, usize);
    define_conversion!(i64, isize);
    define_conversion!(usize, u64);
    define_conversion!(isize, i64);
}

#[cfg(feature = "arch-32")]
mod arch32 {
    use super::*;

    define_conversion!(u32, usize);
    define_conversion!(i32, isize);
    define_conversion!(usize, u32);
    define_conversion!(isize, i32);

    #[cfg(not(feature = "arch-64"))]
    mod non_arch64 {
        use super::*;

        define_conversion!(usize, u64);
        define_conversion!(isize, i64);
    }
}
