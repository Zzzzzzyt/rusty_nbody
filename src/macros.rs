// From wide crate

use std::ops::*;

use crate::Vec3;

macro_rules! bulk_impl_op_ref_self_for {
    ($(($op:ident, $method:ident, $tself:ty) => [$($t:ty),+]),+ $(,)?) => {
      $( // do each trait/list matching given
        $( // do the current trait for each type in its list.
          impl $op<&$t> for $tself {
            type Output = Self;
            #[inline]
            #[must_use]
            fn $method(self, rhs: &$t) -> Self::Output {
              self.$method(*rhs)
            }
          }

          impl $op<$t> for &$tself {
            type Output = $tself;
            #[inline]
            #[must_use]
            fn $method(self, rhs: $t) -> Self::Output {
              (*self).$method(rhs)
            }
          }

          impl $op<&$t> for &$tself {
            type Output = $tself;
            #[inline]
            #[must_use]
            fn $method(self, rhs: &$t) -> Self::Output {
              (*self).$method(*rhs)
            }
          }
        )+
      )+
    };
  }

macro_rules! bulk_impl_op_assign_for {
    ($(($op:ident<$rhs:ty>, $method:ident, $method_assign:ident) => [$($t:ty),+]),+ $(,)?) => {
      $( // do each trait/list matching given
        $( // do the current trait for each type in its list.
          impl $op<$rhs> for $t {
            #[inline]
            fn $method_assign(&mut self, rhs: $rhs) {
              *self = self.$method(rhs);
            }
          }

          impl $op<&$rhs> for $t {
            #[inline]
            fn $method_assign(&mut self, rhs: &$rhs) {
              *self = self.$method(*rhs);
            }
          }
        )+
      )+
    };
  }

bulk_impl_op_ref_self_for! {
    (Add, add, Vec3) => [Vec3],
    (Sub, sub, Vec3) => [Vec3],
    (Mul, mul, Vec3) => [Vec3, f64],
    (Div, div, Vec3) => [Vec3, f64],
}

bulk_impl_op_assign_for! {
    (AddAssign<Vec3>, add, add_assign) => [Vec3],
    (SubAssign<Vec3>, sub, sub_assign) => [Vec3],
    (MulAssign<f64>, mul, mul_assign) => [Vec3],
    (MulAssign<Vec3>, mul, mul_assign) => [Vec3],
    (DivAssign<f64>, div, div_assign) => [Vec3],
    (DivAssign<Vec3>, div, div_assign) => [Vec3],
}
