use crate::rings::ring_trait::Ring;

pub fn shift_ring_vec<R: Ring + Clone>(vec: Vec<R>, shift: usize) -> Vec<R> {
    if shift == 0 {
        return vec;
    }

    let mut coeffs = Vec::with_capacity(vec.len() + shift);
    coeffs.extend(std::iter::repeat_n(R::zero(), shift));
    coeffs.extend(vec);
    coeffs
}
