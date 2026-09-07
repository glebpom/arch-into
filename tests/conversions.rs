use arch_into::{ArchFrom, ArchInto};

macro_rules! conversion_test {
    ($name:ident, $from:ty => $to:ty, $wide:ty, $values:expr) => {
        #[test]
        fn $name() {
            for value in $values {
                let source: $from = value;
                let converted: $to = source.arch_into();
                assert_eq!(
                    converted as $wide, source as $wide,
                    "ArchInto failed to preserve {source}"
                );
                assert_eq!(
                    <$to>::arch_from(source) as $wide,
                    source as $wide,
                    "ArchFrom failed to preserve {source}"
                );
            }
        }
    };
}

fn unsigned_pointer_values() -> Vec<usize> {
    let values = vec![
        usize::MIN,
        1,
        i32::MAX as usize,
        i32::MAX as usize + 1,
        u32::MAX as usize,
        usize::MAX - 1,
        usize::MAX,
    ];

    #[cfg(target_pointer_width = "64")]
    let values = {
        let mut values = values;
        values.push(u32::MAX as usize + 1);
        values
    };

    values
}

fn signed_pointer_values() -> Vec<isize> {
    let values = vec![
        isize::MIN,
        isize::MIN + 1,
        i32::MIN as isize,
        -1,
        0,
        1,
        i32::MAX as isize,
        isize::MAX - 1,
        isize::MAX,
    ];

    #[cfg(target_pointer_width = "64")]
    let values = {
        let mut values = values;
        values.extend([i32::MIN as isize - 1, i32::MAX as isize + 1]);
        values
    };

    values
}

conversion_test!(usize_to_u64, usize => u64, u128, unsigned_pointer_values());
conversion_test!(usize_to_u128, usize => u128, u128, unsigned_pointer_values());
conversion_test!(isize_to_i64, isize => i64, i128, signed_pointer_values());
conversion_test!(isize_to_i128, isize => i128, i128, signed_pointer_values());

conversion_test!(u8_to_usize, u8 => usize, u128, [u8::MIN, 1, u8::MAX]);
conversion_test!(u16_to_usize, u16 => usize, u128, [u16::MIN, 1, u16::MAX]);
conversion_test!(
    u32_to_usize,
    u32 => usize,
    u128,
    [u32::MIN, 1, i32::MAX as u32, i32::MAX as u32 + 1, u32::MAX]
);

conversion_test!(i8_to_isize, i8 => isize, i128, [i8::MIN, -1, 0, 1, i8::MAX]);
conversion_test!(i16_to_isize, i16 => isize, i128, [i16::MIN, -1, 0, 1, i16::MAX]);
conversion_test!(
    i32_to_isize,
    i32 => isize,
    i128,
    [i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX]
);

#[cfg(feature = "no-arch-32")]
conversion_test!(
    u64_to_usize,
    u64 => usize,
    u128,
    [
        u64::MIN,
        1,
        u32::MAX as u64,
        u32::MAX as u64 + 1,
        u64::MAX - 1,
        u64::MAX,
    ]
);

#[cfg(feature = "no-arch-32")]
conversion_test!(
    i64_to_isize,
    i64 => isize,
    i128,
    [
        i64::MIN,
        i64::MIN + 1,
        i32::MIN as i64 - 1,
        i32::MIN as i64,
        -1,
        0,
        1,
        i32::MAX as i64,
        i32::MAX as i64 + 1,
        i64::MAX - 1,
        i64::MAX,
    ]
);

#[cfg(feature = "no-arch-64")]
conversion_test!(usize_to_u32, usize => u32, u128, unsigned_pointer_values());

#[cfg(feature = "no-arch-64")]
conversion_test!(isize_to_i32, isize => i32, i128, signed_pointer_values());
