use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use rug::Integer;

use crate::{impl_add_assign_op, impl_add_op, impl_assign_op, impl_eq, impl_mul_assign_op, impl_mul_op, impl_op, impl_sub_assign_op, impl_sub_op};

use super::super::{group_trait::{AddSupport, AdditiveGroup, AdditiveIdentity, AdditiveInverse, EqSupport, MulSupport, MultiplicativeIdentity, SubSupport}, ring_trait::Ring};

#[derive(Debug, Clone)]
pub struct ZZ(Integer);

impl ZZ {
    pub fn new(n: impl Into<Integer>) -> Self {
        ZZ(n.into())
    }

    fn add_ffn(lhs: &Self, rhs: &Self) -> Self {
        ZZ(lhs.0.clone() + &rhs.0)
    }

    fn add_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        ZZ(lhs.0.clone() + rhs)
    }

    fn add_assign_ffn(lhs: &mut Self, rhs: &Self) {
        lhs.0 += &rhs.0
    }

    fn add_usize_assign_ffn(lhs: &mut Self, rhs: &usize) {
        lhs.0 += rhs
    }

    fn sub_ffn(lhs: &Self, rhs: &Self) -> Self {
        ZZ(lhs.0.clone() - &rhs.0)
    }

    fn sub_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        ZZ(lhs.0.clone() - rhs)
    }

    fn sub_assign_ffn(lhs: &mut Self, rhs: &Self) {
        lhs.0 -= &rhs.0
    }

    fn sub_usize_assign_ffn(lhs: &mut Self, rhs: &usize) {
        lhs.0 -= rhs
    }

    fn mul_ffn(lhs: &Self, rhs: &Self) -> Self {
        ZZ(lhs.0.clone() * &rhs.0)
    }

    fn mul_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        ZZ(lhs.0.clone() * rhs)
    }

    fn mul_assign_ffn(lhs: &mut Self, rhs: &Self) {
        lhs.0 *= &rhs.0
    }

    fn mul_usize_assign_ffn(lhs: &mut Self, rhs: &usize) {
        lhs.0 *= rhs
    }

    fn eq_ffn(lhs: &Self, rhs: &Self) -> bool {
        lhs.0 == rhs.0
    }
}

impl_op!(impl_add_op, ZZ, ZZ, ZZ::add_ffn, []);
impl_op!(impl_add_op, ZZ, usize, ZZ::add_usize_ffn, []);
impl_assign_op!(impl_add_assign_op, ZZ, ZZ, ZZ::add_assign_ffn);
impl_assign_op!(impl_add_assign_op, ZZ, usize, ZZ::add_usize_assign_ffn);

impl_op!(impl_sub_op, ZZ, ZZ, ZZ::sub_ffn, []);
impl_op!(impl_sub_op, ZZ, usize, ZZ::sub_usize_ffn, []);
impl_assign_op!(impl_sub_assign_op, ZZ, ZZ, ZZ::sub_assign_ffn);
impl_assign_op!(impl_sub_assign_op, ZZ, usize, ZZ::sub_usize_assign_ffn);

impl_op!(impl_mul_op, ZZ, ZZ, ZZ::mul_ffn, []);
impl_op!(impl_mul_op, ZZ, usize, ZZ::mul_usize_ffn, []);
impl_assign_op!(impl_mul_assign_op, ZZ, ZZ, ZZ::mul_assign_ffn);
impl_assign_op!(impl_mul_assign_op, ZZ, usize, ZZ::mul_usize_assign_ffn);

impl_eq!(ZZ, ZZ::eq_ffn, []);

impl AddSupport for ZZ {}
impl SubSupport for ZZ {}
impl MulSupport for ZZ {}
impl EqSupport for ZZ {}

impl Neg for ZZ {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ZZ(self.0)
    }
}

impl AdditiveIdentity for ZZ {
    const ZERO: Self = ZZ(Integer::ZERO);

    fn zero() -> Self {
        ZZ(Integer::ZERO)
    }
}

impl MultiplicativeIdentity for ZZ {
    fn one() -> Self {
        ZZ(Integer::from(1))
    }
}

impl AdditiveInverse for ZZ {}
impl AdditiveGroup for ZZ {}
impl Ring for ZZ {}

#[cfg(test)]
mod tests {
    use super::ZZ;

    #[test]
    fn test_zz_eq() {
        let a = ZZ::new(5);
        let b = ZZ::new(8);
        let c = ZZ::new(5);

        assert_eq!(a, c);
        assert_ne!(a, b);
    }
}
