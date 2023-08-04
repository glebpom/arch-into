#[cfg(all(target_pointer_width = "64", feature = "no-arch-64"))]
compile_error!("arch-into is configured to not support 64-bits target");

#[cfg(all(target_pointer_width = "32", feature = "no-arch-32"))]
compile_error!("arch-into is configured to not support 32-bits target");

pub trait ArchInto<T>: Sized {
    fn arch_into(self) -> T;
}

pub trait ArchFrom<T>: Sized {
    fn arch_from(value: T) -> Self;
}

#[allow(unused_macros)]
macro_rules! define_try_conversion {
    ($from:ty, $to:ty, $compare_through:ty) => {
        paste::paste! {
            #[track_caller]
            const fn [<_check_boundaries_ $from _ $to>]() {
                const_panic::concat_assert!{
                    (<$from>::MAX as $compare_through) <= (<$to>::MAX as $compare_through) && (<$from>::MIN as $compare_through) >= (<$to>::MIN as $compare_through),
                    "\n", <$from>::MAX, " <= ", <$to>::MAX, " (", (<$from>::MAX as $compare_through), " <= ", (<$to>::MAX  as $compare_through), ")"
                }

            }
            const [<_CHECK_BOUNDARIES_ $from:upper _ $to:upper>]: () = [<_check_boundaries_ $from _ $to>]();
        }

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

#[cfg(not(any(feature = "no-arch-64", feature = "no-arch-32")))]
mod conversions_64bits_or_32bits {
    // usize/isize is 32bits or 64bits

    use super::*;

    define_try_conversion!(usize, u64, u128);
    define_try_conversion!(isize, i64, i128);

    define_try_conversion!(usize, u128, u128);
    define_try_conversion!(isize, i128, i128);

    define_try_conversion!(u32, usize, u128);
    define_try_conversion!(i32, isize, i128);
}

#[cfg(all(feature = "no-arch-32", not(feature = "no-arch-64")))]
mod conversions_only_64bits {
    // usize/isize is always 64bits

    use super::*;

    define_try_conversion!(usize, u64, u128);
    define_try_conversion!(isize, i64, i128);

    define_try_conversion!(usize, u128, u128);
    define_try_conversion!(isize, i128, i128);

    define_try_conversion!(u64, usize, u128);
    define_try_conversion!(i64, isize, i128);

    define_try_conversion!(u32, usize, u128);
    define_try_conversion!(i32, isize, i128);
}

#[cfg(all(not(feature = "no-arch-32"), feature = "no-arch-64"))]
mod conversions_only_32bits {
    use super::*;

    // usize/isize is always 32bits

    define_try_conversion!(usize, u32, u128);
    define_try_conversion!(isize, i32, i128);

    define_try_conversion!(usize, u64, u128);
    define_try_conversion!(isize, i64, i128);

    define_try_conversion!(usize, u128, u128);
    define_try_conversion!(isize, i128, i128);

    define_try_conversion!(u32, usize, u128);
    define_try_conversion!(i32, isize, i128);
}

define_guaranteed_conversion!(u16, usize);
define_guaranteed_conversion!(i16, isize);

define_guaranteed_conversion!(u8, usize);
define_guaranteed_conversion!(i8, isize);
