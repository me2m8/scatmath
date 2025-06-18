use std::{
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    rc::Rc,
};

use rug::{
    ops::RemRoundingAssign, Integer
};

use crate::{
    impl_add_assign_op, impl_add_op, impl_assign_op, impl_eq, impl_mul_assign_op, impl_mul_op,
    impl_op, impl_sub_assign_op, impl_sub_op,
    rings::{group_trait::{
        AddSupport, AdditiveGroup, AdditiveIdentity, AdditiveInverse, EqSupport, MaybeMultiplicativeInverse, MulSupport, MultiplicativeIdentity, SubSupport
    }, ring_trait::Ring},
};

pub struct ZmodNumber {
    inner: Integer,
    modulus: Option<Rc<Integer>>,
}

/// Implementation of modular arithmetic
impl ZmodNumber {
    pub fn new(mut n: Integer, m: Option<Rc<Integer>>) -> Self {
        if let Some(ref modulus) = m {
            n.rem_euc_assign(modulus.as_ref());
        }

        Self {
            inner: n,
            modulus: m,
        }
    }

    pub fn modulus(&self) -> Option<&Integer> {
        self.modulus.as_deref()
    }

    pub fn inner(&self) -> &Integer {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Integer {
        &mut self.inner
    }

    fn clone_modulus(&self) -> Option<Rc<Integer>> {
        self.modulus.clone()
    }

    fn reduce_w_modulus(&mut self) {
        if self.modulus.is_none() {
            return;
        }
        self.inner.rem_euc_assign(self.modulus.as_deref().unwrap());
    }

    fn add_ffn(lhs: &Self, rhs: &Self) -> Self {
        let modulus = lhs.modulus.clone().or_else(|| rhs.modulus.clone());
        Self::new(lhs.inner().clone() + rhs.inner(), modulus)
    }

    fn add_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        let modulus = lhs.modulus.clone();
        Self::new(lhs.inner().clone() + rhs, modulus)
    }

    fn add_assign_ffn(lhs: &mut Self, rhs: &Self) {
        (*lhs.inner_mut() += rhs.inner());
        lhs.reduce_w_modulus();
    }

    fn add_assign_usize_ffn(lhs: &mut Self, rhs: &usize) {
        (*lhs.inner_mut() += rhs);
        lhs.reduce_w_modulus();
    }

    fn sub_ffn(lhs: &Self, rhs: &Self) -> Self {
        let modulus = lhs.modulus.clone().or_else(|| rhs.modulus.clone());
        Self::new(lhs.inner().clone() - rhs.inner(), modulus)
    }

    fn sub_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        let modulus = lhs.modulus.clone();
        Self::new(lhs.inner().clone() - rhs, modulus)
    }

    fn sub_assign_ffn(lhs: &mut Self, rhs: &Self) {
        (*lhs.inner_mut() -= rhs.inner());
        lhs.reduce_w_modulus();
    }

    fn sub_assign_usize_ffn(lhs: &mut Self, rhs: &usize) {
        (*lhs.inner_mut() -= rhs);
        lhs.reduce_w_modulus();
    }

    fn mul_ffn(lhs: &Self, rhs: &Self) -> Self {
        let modulus = lhs.modulus.clone().or_else(|| rhs.modulus.clone());
        Self::new(lhs.inner().clone() * rhs.inner(), modulus)
    }
    fn mul_usize_ffn(lhs: &Self, rhs: &usize) -> Self {
        let modulus = lhs.modulus.clone();
        Self::new(lhs.inner().clone() * rhs, modulus)
    }

    fn mul_assign_ffn(lhs: &mut Self, rhs: &Self) {
        (*lhs.inner_mut() *= rhs.inner());
        lhs.reduce_w_modulus();
    }

    fn mul_assign_usize_ffn(lhs: &mut Self, rhs: &usize) {
        (*lhs.inner_mut() *= rhs);
        lhs.reduce_w_modulus();
    }

    fn eq_ffn(lhs: &Self, rhs: &Self) -> bool {
        lhs.inner() == rhs.inner()
    }
}

//
// Additive Inverse
//

impl Neg for ZmodNumber {
    type Output = ZmodNumber;

    fn neg(self) -> Self::Output {
        ZmodNumber::new(-self.inner().clone(), self.clone_modulus())
    }
}

impl Neg for &ZmodNumber {
    type Output = ZmodNumber;

    fn neg(self) -> Self::Output {
        ZmodNumber::new(-self.inner().clone(), self.clone_modulus())
    }
}

impl Neg for &mut ZmodNumber {
    type Output = ZmodNumber;

    fn neg(self) -> Self::Output {
        ZmodNumber::new(-self.inner().clone(), self.clone_modulus())
    }
}

impl AdditiveInverse for ZmodNumber {}

//
// Identities
//

impl AdditiveIdentity for ZmodNumber {
    const ZERO: Self = ZmodNumber {
        inner: Integer::ZERO,
        modulus: None,
    };

    fn zero() -> Self {
        Self::ZERO
    }
}

impl MultiplicativeIdentity for ZmodNumber {
    fn one() -> Self {
        Self {
            inner: Integer::from(1),
            modulus: None,
        }
    }
}

//
// Addition
//

impl_op!(impl_add_op, ZmodNumber, ZmodNumber, ZmodNumber::add_ffn, []);
impl_op!(
    impl_add_op,
    ZmodNumber,
    usize,
    ZmodNumber::add_usize_ffn,
    []
);
impl_assign_op!(
    impl_add_assign_op,
    ZmodNumber,
    ZmodNumber,
    ZmodNumber::add_assign_ffn
);
impl_assign_op!(
    impl_add_assign_op,
    ZmodNumber,
    usize,
    ZmodNumber::add_assign_usize_ffn
);
impl AddSupport for ZmodNumber {}

//
// Subtraction
//

impl_op!(impl_sub_op, ZmodNumber, ZmodNumber, ZmodNumber::sub_ffn, []);
impl_op!(
    impl_sub_op,
    ZmodNumber,
    usize,
    ZmodNumber::sub_usize_ffn,
    []
);
impl_assign_op!(
    impl_sub_assign_op,
    ZmodNumber,
    ZmodNumber,
    ZmodNumber::sub_assign_ffn
);
impl_assign_op!(
    impl_sub_assign_op,
    ZmodNumber,
    usize,
    ZmodNumber::sub_assign_usize_ffn
);
impl SubSupport for ZmodNumber {}

//
// Multiplication
//

impl_op!(impl_mul_op, ZmodNumber, ZmodNumber, ZmodNumber::mul_ffn, []);
impl_op!(
    impl_mul_op,
    ZmodNumber,
    usize,
    ZmodNumber::mul_usize_ffn,
    []
);
impl_assign_op!(
    impl_mul_assign_op,
    ZmodNumber,
    ZmodNumber,
    ZmodNumber::mul_assign_ffn
);
impl_assign_op!(
    impl_mul_assign_op,
    ZmodNumber,
    usize,
    ZmodNumber::mul_assign_usize_ffn
);
impl MulSupport for ZmodNumber {}

//
// Equality
//

impl_eq!(ZmodNumber, ZmodNumber::eq_ffn, []);
impl EqSupport for ZmodNumber {}

//
// Maybe Multiplicative Inverse
//

impl MaybeMultiplicativeInverse for ZmodNumber {
    fn inverse(&self) -> Option<Self> {
        unimplemented!();
    }
}

//
// Groups / Rings
//

impl AdditiveGroup for ZmodNumber {}
impl Ring for ZmodNumber {}
