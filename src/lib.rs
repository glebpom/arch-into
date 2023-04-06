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

macro_rules! define_into {
    ($sized:ty, $size:ty) => {
        impl ArchInto<$sized> for $size {
            fn arch_into(self) -> $sized {
                self.try_into().unwrap()
            }
        }
        impl ArchInto<$size> for $sized {
            fn arch_into(self) -> $size {
                self.try_into().unwrap()
            }
        }
        impl ArchFrom<$sized> for $size {
            fn arch_from(value: $sized) -> Self {
                value.try_into().unwrap()
            }
        }
        impl ArchFrom<$size> for $sized {
            fn arch_from(value: $size) -> Self {
                value.try_into().unwrap()
            }
        }
    };
}

macro_rules! define_conversions {
    ($u:ty, $i:ty) => {
        define_into!($u, usize);
        define_into!($i, isize);
    };
}

#[cfg(feature = "arch-64")]
define_conversions!(u64, i64);

#[cfg(feature = "arch-32")]
define_conversions!(u32, i32);
