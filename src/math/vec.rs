use ggez::glam::{IVec2, Vec2};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Vec2Axis {
    X,
    Y,
}

impl Vec2Axis {
    pub fn other(self) -> Vec2Axis {
        match self {
            Vec2Axis::X => Vec2Axis::Y,
            Vec2Axis::Y => Vec2Axis::X,
        }
    }
}

pub trait Vec2CompWise {
    type Primitive;

    #[inline]
    fn get_comp(&self, comp: Vec2Axis) -> Self::Primitive;

    fn get_max_axis(&self) -> Option<Vec2Axis>;

    fn with_offset_on(self, comp: Vec2Axis, offset: Self::Primitive) -> Self;

    fn with_comp(self, comp: Vec2Axis, value: Self::Primitive) -> Self;
}

macro_rules! impl_vec2comp {
    ($name:ty, $prim:ty, $x:expr, $y:expr, $new:expr) => {
        impl Vec2CompWise for $name {
            type Primitive = $prim;

            fn get_comp(&self, comp: Vec2Axis) -> Self::Primitive {
                match comp {
                    Vec2Axis::X => self.$x,
                    Vec2Axis::Y => self.$y,
                }
            }

            fn get_max_axis(&self) -> Option<Vec2Axis> {
                if self.$x > self.$y {
                    Some(Vec2Axis::X)
                } //
                else if self.$y > self.$x {
                    Some(Vec2Axis::Y)
                } //
                else {
                    None
                }
            }

            fn with_offset_on(self, comp: Vec2Axis, offset: Self::Primitive) -> Self {
                match comp {
                    Vec2Axis::X => $new(self.$x + offset, self.$y),
                    Vec2Axis::Y => $new(self.$x, self.$y + offset),
                }
            }

            fn with_comp(self, comp: Vec2Axis, value: Self::Primitive) -> Self {
                match comp {
                    Vec2Axis::X => $new(value, self.$y),
                    Vec2Axis::Y => $new(self.$x, value),
                }
            }
        }
    };
}

impl_vec2comp!(Vec2, f32, x, y, Vec2::new);
impl_vec2comp!(IVec2, i32, x, y, IVec2::new);
