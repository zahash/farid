use crate::Advance;

macro_rules! impl_advance {
    ( $($ty:ty),* ) => {
        $(
            impl Advance for $ty {
                fn advance(&mut self) {
                    *self += 1;
                }
            }
        )*
    };
}

impl_advance!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
