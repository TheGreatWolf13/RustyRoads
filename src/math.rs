use std::ops::Mul;
use crate::float::{F32, F64};

pub trait Sqr: Copy + Mul<Self> {

    #[inline(always)]
    fn sqr(self) -> <Self as Mul<Self>>::Output {
        self * self
    }
}

macro_rules! impl_sqr {
    ($($name:ty),+) => {
        $(
            impl Sqr for $name {}
        )+
    };
}

impl_sqr!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, F32, F64);