use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
pub struct F32(f32);

impl F32 {
    pub fn new(value: f32) -> Self {
        Self(value)
    }
}

impl Deref for F32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for F32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<f32> for F32 {
    fn as_ref(&self) -> &f32 {
        &self.0
    }
}

impl Neg for F32 {
    type Output = F32;

    fn neg(self) -> Self::Output {
        F32(-self.0)
    }
}

impl Neg for &F32 {
    type Output = F32;

    fn neg(self) -> Self::Output {
        F32(-self.0)
    }
}

impl PartialEq for F32 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl PartialOrd for F32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for F32 {}

impl Hash for F32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

macro_rules! impl_binary_expr {
    ($lhs:ty, $rhs:ty, $trait:ident, $name:ident, $i_lhs:ident, $i_rhs:ident, $e:expr) => {
        impl $trait<$rhs> for $lhs {
            type Output = $lhs;

            #[inline]
            #[track_caller]
            fn $name($i_lhs, $i_rhs: $rhs) -> Self::Output {
                $e
            }
        }
        
        forward_ref_binop! { impl $trait, $name for $lhs, $rhs }
    };
}

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }
        
        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }
        
        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            #[track_caller]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    }
}

impl_binary_expr!(F32, F32, Add, add, self, rhs, F32(self.0 + rhs.0));
impl_binary_expr!(F32, F32, Sub, sub, self, rhs, F32(self.0 - rhs.0));
impl_binary_expr!(F32, F32, Mul, mul, self, rhs, F32(self.0 * rhs.0));
impl_binary_expr!(F32, F32, Div, div, self, rhs, F32(self.0 / rhs.0));
impl_binary_expr!(F32, F32, Rem, rem, self, rhs, F32(self.0 % rhs.0));
impl_binary_expr!(F32, f32, Add, add, self, rhs, F32(self.0 + rhs));
impl_binary_expr!(F32, f32, Sub, sub, self, rhs, F32(self.0 - rhs));
impl_binary_expr!(F32, f32, Mul, mul, self, rhs, F32(self.0 * rhs));
impl_binary_expr!(F32, f32, Div, div, self, rhs, F32(self.0 / rhs));
impl_binary_expr!(F32, f32, Rem, rem, self, rhs, F32(self.0 % rhs));
impl_binary_expr!(f32, F32, Add, add, self, rhs, self + rhs.0);
impl_binary_expr!(f32, F32, Sub, sub, self, rhs, self - rhs.0);
impl_binary_expr!(f32, F32, Mul, mul, self, rhs, self * rhs.0);
impl_binary_expr!(f32, F32, Div, div, self, rhs, self / rhs.0);
impl_binary_expr!(f32, F32, Rem, rem, self, rhs, self % rhs.0);

macro_rules! impl_binary_assign {
    ($lhs:ty, $rhs:ty, $trait:ident, $name:ident, $i_lhs:ident, $i_rhs:ident, $e:expr) => {
        impl $trait<$rhs> for $lhs {

            #[inline]
            #[track_caller]
            fn $name(&mut $i_lhs, $i_rhs: $rhs) {
                $e
            }
        }
        
        forward_ref_op_assign! { impl $trait, $name for $lhs, $rhs }
    };
}

macro_rules! forward_ref_op_assign {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {

        impl $imp<&$u> for $t {
            #[inline]
            #[track_caller]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    }
}

impl_binary_assign!(F32, F32, AddAssign, add_assign, self, rhs, self.0 += rhs.0);
impl_binary_assign!(F32, F32, SubAssign, sub_assign, self, rhs, self.0 -= rhs.0);
impl_binary_assign!(F32, F32, MulAssign, mul_assign, self, rhs, self.0 *= rhs.0);
impl_binary_assign!(F32, F32, DivAssign, div_assign, self, rhs, self.0 /= rhs.0);
impl_binary_assign!(F32, F32, RemAssign, rem_assign, self, rhs, self.0 %= rhs.0);
impl_binary_assign!(F32, f32, AddAssign, add_assign, self, rhs, self.0 += rhs);
impl_binary_assign!(F32, f32, SubAssign, sub_assign, self, rhs, self.0 -= rhs);
impl_binary_assign!(F32, f32, MulAssign, mul_assign, self, rhs, self.0 *= rhs);
impl_binary_assign!(F32, f32, DivAssign, div_assign, self, rhs, self.0 /= rhs);
impl_binary_assign!(F32, f32, RemAssign, rem_assign, self, rhs, self.0 %= rhs);
impl_binary_assign!(f32, F32, AddAssign, add_assign, self, rhs, *self += rhs.0);
impl_binary_assign!(f32, F32, SubAssign, sub_assign, self, rhs, *self -= rhs.0);
impl_binary_assign!(f32, F32, MulAssign, mul_assign, self, rhs, *self *= rhs.0);
impl_binary_assign!(f32, F32, DivAssign, div_assign, self, rhs, *self /= rhs.0);
impl_binary_assign!(f32, F32, RemAssign, rem_assign, self, rhs, *self %= rhs.0);

#[derive(Debug, Copy, Clone)]
pub struct F64(f64);

impl F64 {
    pub fn new(value: f64) -> Self {
        Self(value)
    }
}

impl Deref for F64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for F64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<f64> for F64 {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl Neg for F64 {
    type Output = F64;

    fn neg(self) -> Self::Output {
        F64(-self.0)
    }
}

impl Neg for &F64 {
    type Output = F64;

    fn neg(self) -> Self::Output {
        F64(-self.0)
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for F64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for F64 {}

impl Hash for F64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state)
    }
}

impl_binary_expr!(F64, F64, Add, add, self, rhs, F64(self.0 + rhs.0));
impl_binary_expr!(F64, F64, Sub, sub, self, rhs, F64(self.0 - rhs.0));
impl_binary_expr!(F64, F64, Mul, mul, self, rhs, F64(self.0 * rhs.0));
impl_binary_expr!(F64, F64, Div, div, self, rhs, F64(self.0 / rhs.0));
impl_binary_expr!(F64, F64, Rem, rem, self, rhs, F64(self.0 % rhs.0));
impl_binary_expr!(F64, f64, Add, add, self, rhs, F64(self.0 + rhs));
impl_binary_expr!(F64, f64, Sub, sub, self, rhs, F64(self.0 - rhs));
impl_binary_expr!(F64, f64, Mul, mul, self, rhs, F64(self.0 * rhs));
impl_binary_expr!(F64, f64, Div, div, self, rhs, F64(self.0 / rhs));
impl_binary_expr!(F64, f64, Rem, rem, self, rhs, F64(self.0 % rhs));
impl_binary_expr!(f64, F64, Add, add, self, rhs, self + rhs.0);
impl_binary_expr!(f64, F64, Sub, sub, self, rhs, self - rhs.0);
impl_binary_expr!(f64, F64, Mul, mul, self, rhs, self * rhs.0);
impl_binary_expr!(f64, F64, Div, div, self, rhs, self / rhs.0);
impl_binary_expr!(f64, F64, Rem, rem, self, rhs, self % rhs.0);

impl_binary_assign!(F64, F64, AddAssign, add_assign, self, rhs, self.0 += rhs.0);
impl_binary_assign!(F64, F64, SubAssign, sub_assign, self, rhs, self.0 -= rhs.0);
impl_binary_assign!(F64, F64, MulAssign, mul_assign, self, rhs, self.0 *= rhs.0);
impl_binary_assign!(F64, F64, DivAssign, div_assign, self, rhs, self.0 /= rhs.0);
impl_binary_assign!(F64, F64, RemAssign, rem_assign, self, rhs, self.0 %= rhs.0);
impl_binary_assign!(F64, f64, AddAssign, add_assign, self, rhs, self.0 += rhs);
impl_binary_assign!(F64, f64, SubAssign, sub_assign, self, rhs, self.0 -= rhs);
impl_binary_assign!(F64, f64, MulAssign, mul_assign, self, rhs, self.0 *= rhs);
impl_binary_assign!(F64, f64, DivAssign, div_assign, self, rhs, self.0 /= rhs);
impl_binary_assign!(F64, f64, RemAssign, rem_assign, self, rhs, self.0 %= rhs);
impl_binary_assign!(f64, F64, AddAssign, add_assign, self, rhs, *self += rhs.0);
impl_binary_assign!(f64, F64, SubAssign, sub_assign, self, rhs, *self -= rhs.0);
impl_binary_assign!(f64, F64, MulAssign, mul_assign, self, rhs, *self *= rhs.0);
impl_binary_assign!(f64, F64, DivAssign, div_assign, self, rhs, *self /= rhs.0);
impl_binary_assign!(f64, F64, RemAssign, rem_assign, self, rhs, *self %= rhs.0);