use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use itertools::Itertools;

use crate::{
    impl_add_assign_op, impl_add_op, impl_eq, impl_mul_assign_op, impl_mul_op, impl_op,
    impl_sub_assign_op, impl_sub_op,
    rings::{
        group_trait::{AddSupport, AdditiveGroup, AdditiveIdentity, AdditiveInverse, EqSupport, MulSupport, SubSupport},
        ring_trait::Ring,
    },
    utils::utils::shift_ring_vec,
};

#[derive(Debug, Clone)]
pub struct Polynomial<R>
where
    R: Ring + Clone,
{
    coefficients: Vec<R>,
}

impl<R: Ring + Clone> Polynomial<R> {
    pub fn new() -> Self {
        Self {
            coefficients: vec![],
        }
    }

    pub fn from_owned_coefficients(mut coefficients: Vec<R>) -> Self {
        let zero = R::zero();

        while let Some(a) = coefficients.last() {
            if a != &zero {
                break;
            }

            coefficients.pop();
        }

        Self { coefficients }
    }

    pub fn from_coefficients(coefficients: &[impl Into<R> + Clone]) -> Self {
        let mut coeffs: Vec<_> = coefficients.iter().map(|r| r.clone().into()).collect();
        let zero = R::zero();
        while let Some(a) = coeffs.last() {
            if a != &zero {
                break;
            }

            coeffs.pop();
        }

        Self {
            coefficients: coeffs,
        }
    }

    pub fn shift_by(&mut self, x_deg: usize) {
        if x_deg == 0 || self.is_zero() {
            return;
        }

        let mut new_coeffs = Vec::with_capacity(self.coefficients.len() + x_deg);
        new_coeffs.extend(std::iter::repeat_n(R::zero(), x_deg));
        new_coeffs.append(&mut self.coefficients);
        self.coefficients = new_coeffs;
    }

    pub fn shifted_by(&self, x_deg: usize) -> Self {
        if x_deg == 0 || self.is_zero() {
            return self.clone();
        }

        let mut coeffs = Vec::with_capacity(self.degree() + 1 + x_deg);
        coeffs[x_deg..].clone_from_slice(&self.coefficients);
        Self::from_owned_coefficients(coeffs)
    }

    /// Returns a reference to the coefficients of this [`Polynomial<R>`].
    pub fn coefficients(&self) -> &[R] {
        &self.coefficients
    }

    pub fn coefficient(&self, i: usize) -> Option<R> {
        self.coefficients.get(i).cloned()
    }

    /// Returns the degree of this [`Polynomial<R>`].
    pub fn degree(&self) -> usize {
        (self.coefficients.len() - 1).max(0)
    }

    /// Returns the constant of this [`Polynomial<R>`].
    pub fn constant(&self) -> R {
        if self.coefficients.is_empty() {
            R::zero()
        } else {
            self.coefficients[0].clone()
        }
    }

    /// Returns whether this [`Polynomial<R>`] is zero.
    pub fn is_zero(&self) -> bool {
        self.coefficients.is_empty()
            || (self.coefficients.len() == 1 && self.coefficients[0] == R::zero())
    }

    pub fn is_linear(&self) -> bool {
        self.degree() == 1
    }

    //
    // Addition
    //

    fn add_ffn(lhs: &Self, rhs: &Self) -> Self {
        if lhs.is_zero() {
            return rhs.clone();
        }
        if rhs.is_zero() {
            return lhs.clone();
        }

        let (short, long) = if lhs.coefficients.len() >= rhs.coefficients.len() {
            (&rhs.coefficients, &lhs.coefficients)
        } else {
            (&lhs.coefficients, &rhs.coefficients)
        };

        let mut coeffs = long.clone();
        short.iter().enumerate().for_each(|(i, c)| coeffs[i] += c);

        Self::from_owned_coefficients(coeffs)
    }

    fn add_assign_ffn(lhs: &mut Self, rhs: &Self) {
        if lhs.coefficients.len() < rhs.coefficients.len() {
            lhs.coefficients.resize(rhs.coefficients.len(), R::zero());
        }

        rhs.coefficients
            .iter()
            .enumerate()
            .for_each(|(i, c)| lhs.coefficients[i] += c);
    }

    //
    // Subtraction
    //

    fn sub_ffn(lhs: &Self, rhs: &Self) -> Self {
        if lhs.is_zero() {
            return -rhs.clone();
        }

        let mut out = lhs.clone();

        if out.coefficients.len() < rhs.coefficients.len() {
            out.coefficients.resize(rhs.coefficients.len(), R::zero());
        }

        rhs.coefficients
            .iter()
            .enumerate()
            .for_each(|(i, c)| out.coefficients[i] -= c);

        out
    }

    fn sub_assign_ffn(lhs: &mut Self, rhs: &Self) {
        if lhs.coefficients.len() < rhs.coefficients.len() {
            lhs.coefficients.resize(rhs.coefficients.len(), R::zero());
        }

        rhs.coefficients
            .iter()
            .enumerate()
            .for_each(|(i, c)| lhs.coefficients[i] -= c);
    }

    //
    // Scalar Multiplication
    //

    fn scalar_mul_ffn(lhs: &Self, scalar: &R) -> Self {
        if *scalar == R::zero() {
            return Self::new();
        }

        let mut coeffs = lhs.coefficients.clone();
        (0..coeffs.len()).for_each(|i| coeffs[i] *= scalar);
        Self::from_owned_coefficients(coeffs)
    }

    fn scalar_mul_assign_ffn(lhs: &mut Self, scalar: &R) {
        if *scalar == R::zero() {
            *lhs = Self::new();
            return;
        }

        (0..lhs.coefficients.len()).for_each(|i| lhs.coefficients[i] *= scalar);
    }

    //
    // Polynomial Multiplication
    //

    /// Function for multiplying two polynomials.
    fn polynomial_mul_ffn(lhs: &Self, rhs: &Self) -> Self {
        match (lhs.coefficients().len(), rhs.coefficients().len()) {
            (0, _) => Polynomial::new(),
            (_, 0) => Polynomial::new(),
            (1, _) => rhs * &lhs.coefficients()[0],
            (_, 1) => lhs * &rhs.coefficients()[0],
            (2, 2) => {
                let a = &lhs.coefficients[0];
                let b = &lhs.coefficients[1];
                let c = &rhs.coefficients[0];
                let d = &rhs.coefficients[1];
                let ac = a.clone() * c;
                let ad = a.clone() * d;
                let bc = b.clone() * c;
                let bd = b.clone() * d;

                Polynomial::from_owned_coefficients(vec![ac, ad + bc, bd])
            }
            (d, d_) => {
                let n = d.max(d_).next_power_of_two();

                let mut left_coeff = lhs.coefficients.clone();
                let mut right_coeff = rhs.coefficients.clone();
                left_coeff.resize(n, R::zero());
                right_coeff.resize(n, R::zero());

                let res_coeff = Polynomial::poly_mul_internal(&left_coeff, &right_coeff);
                Polynomial::from_owned_coefficients(res_coeff)
            }
        }
    }

    fn polynomial_mul_assign_ffn(lhs: &mut Self, rhs: &Self) {
        let new = &*lhs * rhs;
        *lhs = new
    }

    /// Function for multiplying two polynomials by their coefficients. Assumes that the
    /// coefficient vectors have the same length, and are not empty.
    fn poly_mul_internal(lhs: &[R], rhs: &[R]) -> Vec<R> {
        match (lhs.len(), rhs.len()) {
            (0, _) => vec![],
            (1, _) => vec![lhs[0].clone() * &rhs[0]],
            (2, _) => {
                let a = &lhs[0];
                let b = &lhs[1];
                let c = &rhs[0];
                let d = &rhs[1];
                let ac = a.clone() * c;
                let ad = a.clone() * d;
                let bc = b.clone() * c;
                let bd = b.clone() * d;

                vec![ac, ad + bc, bd]
            }
            (d, _) => {
                // Karatsuba polynomial multiplication: O(n^1.6)
                let k = d / 2;

                let (p0, p1) = lhs.split_at(k);
                let (q0, q1) = rhs.split_at(k);

                let p0q0 = Polynomial::poly_mul_internal(p0, q0);
                let mut p1q0 = Polynomial::poly_mul_internal(p1, q0);
                let p0q1 = Polynomial::poly_mul_internal(p0, q1);
                let p1q1 = Polynomial::poly_mul_internal(p1, q1);

                p0q1.iter().enumerate().for_each(|(i, c)| p1q0[i] += c);

                let p1q0_p0q1 = shift_ring_vec(p1q0, k);
                let p1q1 = shift_ring_vec(p1q1, d);

                let len = p0q0.len().max(p1q0_p0q1.len()).max(p1q1.len());
                let zero = R::ZERO; // To avoid some unnecessary clones

                (0..len)
                    .map(|i| {
                        let a = p0q0.get(i).cloned().unwrap_or_else(R::zero);
                        let b = p1q0_p0q1.get(i).unwrap_or(&zero);
                        let c = p1q1.get(i).unwrap_or(&zero);

                        a + b + c
                    })
                    .collect_vec()
            }
        }
    }

    //
    // Polynomial Remainder
    //

    fn ref_polynomial_remainder_ffn(lhs: &Self, rhs: &Self) -> Self {
        let mut a = lhs.coefficients().to_vec();
        let b = rhs.coefficients();

        Polynomial::polynomial_remainder_internal(&mut a, b);
        Polynomial::from_owned_coefficients(a)
    }

    fn owned_polynomial_remainder_ffn(lhs: Self, rhs: &Self) -> Self {
        let mut a = lhs.coefficients;
        let b = rhs.coefficients();

        Polynomial::polynomial_remainder_internal(&mut a, b);
        Polynomial::from_owned_coefficients(a)
    }

    fn polynomial_remainder_assign_ffn(lhs: &mut Self, rhs: &Self) {
        let b = rhs.coefficients();
        Polynomial::polynomial_remainder_internal(&mut lhs.coefficients, b);
    }

    fn polynomial_remainder_internal(a: &mut Vec<R>, b: &[R]) {
        let deg_b = b.len() - 1;
        let lc_b = b[deg_b].clone();

        while a.len() >= b.len() {
            let deg_a = a.len() - 1;
            let coeff_a = a[deg_a].clone();

            for c in a.iter_mut() {
                *c *= &lc_b;
            }

            let factor = coeff_a;
            let shift = deg_a - deg_b;

            (0..=deg_b).for_each(|i| {
                let j = i + shift;
                a[j] -= b[i].clone() * &factor;
            });

            while a.last().is_some_and(|c| *c == R::ZERO) {
                a.pop();
            }
        }
    }

    //
    // Equality
    //

    fn eq_ffn(lhs: &Self, rhs: &Self) -> bool {
        lhs.coefficients().len() == rhs.coefficients().len()
            && lhs
                .coefficients()
                .iter()
                .zip(rhs.coefficients().iter())
                .all(|(a, b)| a == b)
    }

    
}

impl<R: Ring + Clone> Default for Polynomial<R> {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! impl_polynomial_assign_op {
    (
        $op_assign_impl:ident,
        $rhs:ty,
        $func:path
    ) => {
        $op_assign_impl!(type = Polynomial<R>, rhs = $rhs, func = $func, bounds = [R: Ring + Clone]);
        $op_assign_impl!(type = Polynomial<R>, rhs = &$rhs, func = $func, bounds = [R: Ring + Clone]);
        $op_assign_impl!(type = Polynomial<R>, rhs = &mut $rhs, func = $func, bounds = [R: Ring + Clone]);
        $op_assign_impl!(type = &mut Polynomial<R>, rhs = $rhs, func = $func, bounds = [R: Ring + Clone]);
        $op_assign_impl!(type = &mut Polynomial<R>, rhs = &$rhs, func = $func, bounds = [R: Ring + Clone]);
        $op_assign_impl!(type = &mut Polynomial<R>, rhs = &mut $rhs, func = $func, bounds = [R: Ring + Clone]);
    };
}

//
// Addition
//

impl_op!(impl_add_op, Polynomial<R>, Polynomial<R>, Polynomial::add_ffn, [R: Ring + Clone]);
impl_polynomial_assign_op!(
    impl_add_assign_op,
    Polynomial<R>,
    Polynomial::add_assign_ffn
);
impl<R: Ring + Clone> AddSupport for Polynomial<R> {}

//
// Subtraction
//

impl_op!(impl_sub_op, Polynomial<R>, Polynomial<R>, Polynomial::sub_ffn, [R: Ring + Clone]);
impl_polynomial_assign_op!(
    impl_sub_assign_op,
    Polynomial<R>,
    Polynomial::sub_assign_ffn
);
impl<R: Ring + Clone> SubSupport for Polynomial<R> {}

//
// Negation
//

impl<R: Ring + Clone> Neg for Polynomial<R> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for coeff in self.coefficients.iter_mut() {
            *coeff = -coeff.clone();
        }

        self
    }
}

impl<R: Ring + Clone> Neg for &Polynomial<R> {
    type Output = Polynomial<R>;

    fn neg(self) -> Self::Output {
        let mut out =
            Polynomial::from_owned_coefficients(Vec::with_capacity(self.coefficients.len()));

        for i in 0..self.coefficients.len() {
            out.coefficients.push(-self.coefficients[i].clone());
        }

        out
    }
}

impl<R: Ring + Clone> Neg for &mut Polynomial<R> {
    type Output = Polynomial<R>;

    fn neg(self) -> Self::Output {
        let mut out =
            Polynomial::from_owned_coefficients(Vec::with_capacity(self.coefficients.len()));

        for i in 0..self.coefficients.len() {
            out.coefficients.push(-self.coefficients[i].clone());
        }

        out
    }
}

impl<R: Ring + Clone> AdditiveInverse for Polynomial<R> {}

//
// Scalar Multiplication
//

impl_op!(impl_mul_op, Polynomial<R>, R, Polynomial::scalar_mul_ffn, [R: Ring + Clone]);
impl_polynomial_assign_op!(impl_mul_assign_op, R, Polynomial::scalar_mul_assign_ffn);

//
// Polynomial Multiplication
//

impl_op!(impl_mul_op, Polynomial<R>, Polynomial<R>, Polynomial::polynomial_mul_ffn, [R: Ring + Clone]);
impl_polynomial_assign_op!(impl_mul_assign_op, Polynomial<R>, Polynomial::polynomial_mul_assign_ffn);
impl<R: Ring + Clone> MulSupport for Polynomial<R> {}

//
// Polynomial Remainder
//

macro_rules! impl_rem_op {
    ($type:ty, $rhs:ty, $out:ty, $ffn:path, [$($bounds:tt)*]) => {
        impl<$($bounds)*> Rem<$rhs> for $type {
            type Output = $out;

            fn rem(self, rhs: $rhs) -> Self::Output {
                $ffn(self, &rhs)
            }
        }
    };
}

impl_rem_op!(Polynomial<R>, Polynomial<R>, Polynomial<R>, Polynomial::owned_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(Polynomial<R>, &Polynomial<R>, Polynomial<R>, Polynomial::owned_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(Polynomial<R>, &mut Polynomial<R>, Polynomial<R>, Polynomial::owned_polynomial_remainder_ffn, [R: Ring + Clone]);

impl_rem_op!(&Polynomial<R>, Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(&Polynomial<R>, &Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(&Polynomial<R>, &mut Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);

impl_rem_op!(&mut Polynomial<R>, Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(&mut Polynomial<R>, &Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);
impl_rem_op!(&mut Polynomial<R>, &mut Polynomial<R>, Polynomial<R>, Polynomial::ref_polynomial_remainder_ffn, [R: Ring + Clone]);

//
// Equality
//

impl_eq!(Polynomial<R>, Polynomial<R>::eq_ffn, [R: Ring + Clone]);
impl<R: Ring + Clone> EqSupport for Polynomial<R> {}

impl<R> AdditiveIdentity for Polynomial<R>
where
    R: Ring + Clone,
{
    const ZERO: Self = Self {
        coefficients: vec![], 
    };

    fn zero() -> Self {
        Self::new()
    }
}

impl<R: Ring + Clone> AdditiveGroup for Polynomial<R> {}
impl<R: Ring + Clone> Ring for Polynomial<R> {}
