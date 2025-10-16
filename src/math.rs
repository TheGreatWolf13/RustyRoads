use crate::float::{F32, F64};
use macro_pub::macro_pub;
use std::ops::Mul;
use ggez::glam::Vec2;

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

#[macro_pub(crate)]
macro_rules! if_else {
    ($condition:expr => $true_value:expr ; $false_value:expr) => {
        if $condition {
            $true_value
        } //
        else {
            $false_value
        }
    };
}

#[derive(Copy, Clone, PartialEq)]
pub struct NormalizedVec2(Vec2);

impl From<Vec2> for NormalizedVec2 {
    fn from(vec: Vec2) -> Self {
        Self(vec.normalize())
    }
}