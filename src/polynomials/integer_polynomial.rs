use itertools::Itertools;
use rug::Integer;
use std::ops::Mul;

use crate::{impl_mul_op, impl_op, rings::integer_ring::integer_ring::ZZ};

use super::generic_polynomial::Polynomial;

impl Polynomial<ZZ> {
    fn from_integer_coefficients(coeffs: Vec<impl Into<Integer>>) -> Self {
        let coeffs: Vec<ZZ> = coeffs.into_iter().map(|c| ZZ::new(c)).collect_vec();
        Polynomial::from_owned_coefficients(coeffs)
    }

    fn scalar_mul_usize_ffn(lhs: &Self, scalar: &usize) -> Self {
        if *scalar == 0 {
            return Self::new();
        }

        let mut coeffs = lhs.coefficients().to_vec();
        (0..coeffs.len()).for_each(|i| coeffs[i] *= scalar);
        Self::from_owned_coefficients(coeffs)
    }
}

impl_op!(impl_mul_op, Polynomial<ZZ>, usize, Polynomial<ZZ>::scalar_mul_usize_ffn, []);

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{
        polynomials::generic_polynomial::Polynomial,
        rings::{group_trait::AdditiveIdentity, integer_ring::integer_ring::ZZ},
    };
    fn int_vec(vec: &[i64]) -> Vec<ZZ> {
        vec.iter().map(|x| ZZ::new(*x)).collect_vec()
    }

    #[test]
    fn test_stripping_zeros() {
        let coeffs = vec![
            ZZ::new(0),
            ZZ::new(1),
            ZZ::new(5),
            ZZ::new(3),
            ZZ::ZERO,
            ZZ::ZERO,
        ];
        let poly = Polynomial::from_owned_coefficients(coeffs);

        assert_eq!(poly.coefficients(), &int_vec(&[0, 1, 5, 3]))
    }

    #[test]
    fn test_add_polynomials() {
        let c1 = int_vec(&[0, 1, 3, 5, 7]);
        let c2 = int_vec(&[1, 4, 6, 7, 2, 1, 5, 0, 5]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res = p1 + p2;

        assert_eq!(res.coefficients(), &int_vec(&[1, 5, 9, 12, 9, 1, 5, 0, 5]));
    }

    #[test]
    fn test_sub_polynomials() {
        let c1 = int_vec(&[0, 1, 3, 5, 7]);
        let c2 = int_vec(&[1, 4, 6, 7, 2, 1, 5, 0, 5]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res = p1 - p2;

        assert_eq!(
            res.coefficients(),
            &int_vec(&[-1, -3, -3, -2, 5, -1, -5, 0, -5])
        );
    }

    #[test]
    fn test_scalar_mul() {
        let c1 = int_vec(&[9, 4, 6, 1, 8]);

        let p1 = Polynomial::from_owned_coefficients(c1);

        let res1 = &p1 * ZZ::new(2);
        let res2 = &p1 * 3usize;

        assert_eq!(res1.coefficients(), int_vec(&[18, 8, 12, 2, 16]));
        assert_eq!(res2.coefficients(), int_vec(&[27, 12, 18, 3, 24]));
    }

    #[test]
    fn test_polynomial_zero_zz_mul() {
        let c1 = int_vec(&[1, 2, 3, 4]);
        let p1 = Polynomial::from_owned_coefficients(c1);

        let res1 = &p1 * ZZ::new(0);

        assert_eq!(res1.coefficients(), Vec::<ZZ>::new());
    }

    #[test]
    fn test_polynomial_zero_usize_mul() {
        let c1 = int_vec(&[1, 2, 3, 4]);
        let p1 = Polynomial::from_owned_coefficients(c1);

        #[allow(clippy::erasing_op)]
        let res1 = &p1 * 0;

        assert_eq!(res1.coefficients(), Vec::<ZZ>::new());
    }

    #[test]
    fn test_polynomial_zero_poly_mul() {
        let c1 = int_vec(&[1, 2, 3, 4]);
        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(vec![ZZ::new(0)]); // Will be empty

        let res1 = &p1 * &p2;
        let res2 = &p2 * &p1;

        assert_eq!(res1.coefficients(), Vec::<ZZ>::new());
        assert_eq!(res2.coefficients(), Vec::<ZZ>::new());
    }

    #[test]
    fn test_same_polynomial_eq() {
        let c1 = int_vec(&[1, 3, 5, 7]);

        let p1 = Polynomial::from_owned_coefficients(c1.clone());
        let p2 = Polynomial::from_owned_coefficients(c1.clone());

        assert_eq!(p1, p2);
    }

    #[test]
    fn test_different_deg_polynomial_eq() {
        let c1 = int_vec(&[1, 3, 5, 7]);
        let c2 = int_vec(&[1, 3]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        assert_ne!(p1, p2);
    }

    #[test]
    fn test_different_coeff_polynomial_eq() {
        let c1 = int_vec(&[1, 3, 5, 7]);
        let c2 = int_vec(&[1, 3, 5, 8]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        assert_ne!(p1, p2);
    }

    #[test]
    fn test_polynomial_deg_0_mul() {
        let c1 = int_vec(&[1, 2, 4, 8]);
        let c2 = int_vec(&[3]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res1 = &p1 * &p2;
        let res2 = &p2 * &p1;

        assert_eq!(res1, res2);
        assert_eq!(res1.coefficients(), int_vec(&[3, 6, 12, 24]));
    }

    #[test]
    fn test_polynomial_deg_1_mul() {
        let c1 = int_vec(&[1, 2]);
        let c2 = int_vec(&[3, 5]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res1 = &p1 * &p2;
        let res2 = &p2 * &p1;

        assert_eq!(res1, res2);
        assert_eq!(res1.coefficients(), int_vec(&[3, 11, 10]));
    }

    #[test]
    fn test_polynomial_deg_2_mul() {
        let c1 = int_vec(&[1, 2, 3]);
        let c2 = int_vec(&[4, 5, 6]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res = &p1 * &p2;
        assert_eq!(res.coefficients(), int_vec(&[4, 13, 28, 27, 18]))
    }

    #[test]
    fn test_polynomial_high_same_deg_mul() {
        let c1 = int_vec(&[1, 2, 5, 6, 8, 3, 1]);
        let c2 = int_vec(&[3, 5, 0, 1, 5, 9, 2]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res1 = &p1 * &p2;
        let res2 = &p2 * &p1;

        assert_eq!(res1, res2);
        assert_eq!(res1.coefficients(), int_vec(&[3, 11, 25, 44, 61, 73, 69, 92, 107, 100, 48, 15, 2]));
    }

    #[test]
    fn test_polynomial_different_length_mul() {
        let c1 = int_vec(&[1, 2, 3, 5, 6, 7]);
        let c2 = int_vec(&[7, 8, 9]);

        let p1 = Polynomial::from_owned_coefficients(c1);
        let p2 = Polynomial::from_owned_coefficients(c2);

        let res = &p1 * &p2;

        assert_eq!(res.coefficients(), int_vec(&[7, 22, 46, 77, 109, 142, 110, 63]));
    }
}
