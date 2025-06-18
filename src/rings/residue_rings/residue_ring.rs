use std::{error::Error, rc::Rc};

use rug::Integer;

use super::residue_ring_number::ZmodNumber;

pub struct Zmod {
    modulus: Rc<Integer>,
}

impl Zmod {
    pub fn new(modulus: impl Into<Integer>) -> Result<Self, Box<dyn Error>> {
        let m = modulus.into();

        if m <= 0 {
            Err("Modulus must be positive".into())
        } else {
            Ok(Self { modulus: Rc::new(m) })
        }
    }

    #[inline]
    pub fn modulus(&self) -> &Integer {
        &self.modulus
    }

    pub fn number(&self, n: impl Into<Integer>) -> ZmodNumber {
        let n = n.into();
        ZmodNumber::new(n, Some(self.modulus.clone()))
    }

    pub fn zero(&self) -> ZmodNumber {
        ZmodNumber::new(Integer::ZERO, Some(self.modulus.clone()))
    }

    pub fn one(&self) -> ZmodNumber {
        ZmodNumber::new(Integer::from(1), Some(self.modulus.clone()))
    }
}
