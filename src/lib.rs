#[cfg(all(target_pointer_width = "32", feature = "arch-64"))]
compile_error!("arch-into is configured to support 64-bits, but your target architecture is 32-bits");

#[cfg(all(target_pointer_width = "16", feature = "arch-64"))]
compile_error!("arch-into is configured to support 64-bits, but your target architecture is 16-bits");

#[cfg(all(target_pointer_width = "16", feature = "arch-32"))]
compile_error!("arch-into is configured to support 32-bits, but your target architecture is 16-bits");

pub trait ArchInto<T>: Sized {
    fn arch_into(self) -> T;
}

pub trait ArchFrom<T>: Sized {
    fn arch_from(value: T) -> Self;
}

#[allow(unused_macros)]
macro_rules! define_try_conversion {
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

macro_rules! define_guaranteed_conversion {
    ($from:ty, $to:ty) => {
        impl ArchInto<$to> for $from {
            fn arch_into(self) -> $to {
                self.into()
            }
        }
        impl ArchFrom<$from> for $to {
            fn arch_from(value: $from) -> Self {
                value.into()
            }
        }
    };
}

#[cfg(all(feature = "arch-64", feature = "arch-32"))]
mod conversions_64bits_or_32bits {
    // usize/isize is 32bits or 64bits

    use super::*;

    define_try_conversion!(usize, u64);
    define_try_conversion!(isize, i64);

    define_try_conversion!(usize, u128);
    define_try_conversion!(isize, i128);

    define_try_conversion!(u32, usize);
    define_try_conversion!(i32, isize);
}

#[cfg(all(feature = "arch-64", not(feature = "arch-32")))]
mod conversions_only_64bits {
    // usize/isize is always 64bits

    use super::*;

    define_try_conversion!(usize, u64);
    define_try_conversion!(isize, i64);

    define_try_conversion!(usize, u128);
    define_try_conversion!(isize, i128);

    define_try_conversion!(u64, usize);
    define_try_conversion!(i64, isize);

    define_try_conversion!(u32, usize);
    define_try_conversion!(i32, isize);
}

#[cfg(all(not(feature = "arch-64"), feature = "arch-32"))]
mod conversions_only_32bits {
    use super::*;

    // usize/isize is always 32bits

    define_try_conversion!(usize, u32);
    define_try_conversion!(isize, i32);

    define_try_conversion!(usize, u64);
    define_try_conversion!(isize, i64);

    define_try_conversion!(usize, u128);
    define_try_conversion!(isize, i128);

    define_try_conversion!(u32, usize);
    define_try_conversion!(i32, isize);
}

define_guaranteed_conversion!(u16, usize);
define_guaranteed_conversion!(i16, isize);

define_guaranteed_conversion!(u8, usize);
define_guaranteed_conversion!(i8, isize);
