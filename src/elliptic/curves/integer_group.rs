use crate::{BigInt, ErrorKey};
use std::ops::{Add,Mul};
use crate::elliptic::curves::traits::{ECScalar, ECPoint};
use p256::Scalar;

pub struct Zqg{
    g:BigInt,
    q:BigInt,
}
pub struct Zqf{
    f:BigInt,
    q:BigInt,
}
pub type GE = Zqg;
pub type FE = Zqf;

impl ECScalar for Zqf{
    type SecretKey = BigInt;

    fn new_random() -> Self {
        unimplemented!()
    }

    fn zero() -> Self {
        unimplemented!()
    }

    fn get_element(&self) -> Self::SecretKey {
        unimplemented!()
    }

    fn set_element(&mut self, element: Self::SecretKey) {
        unimplemented!()
    }

    fn from(n: &BigInt) -> Self {
        unimplemented!()
    }

    fn to_big_int(&self) -> BigInt {
        unimplemented!()
    }

    fn q() -> BigInt {
        unimplemented!()
    }

    fn add(&self, other: &Self::SecretKey) -> Self {
        unimplemented!()
    }

    fn mul(&self, other: &Self::SecretKey) -> Self {
        unimplemented!()
    }

    fn sub(&self, other: &Self::SecretKey) -> Self {
        unimplemented!()
    }

    fn invert(&self) -> Self {
        unimplemented!()
    }
}
impl Mul<Zqf> for Zqf {
    type Output = Zqf;
    fn mul(self, other: Zqf) -> Zqf {
        (&self).mul(&other.get_element())
    }
}

impl<'o> Mul<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn mul(self, other: &'o Zqf) -> Zqf {
        (&self).mul(&other.get_element())
    }
}

impl Add<Zqf> for Zqf {
    type Output = Zqf;
    fn add(self, other: Zqf) -> Zqf {
        (&self).add(&other.get_element())
    }
}

impl<'o> Add<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn add(self, other: &'o Zqf) -> Zqf {
        (&self).add(&other.get_element())
    }
}
impl PartialEq for Zqf {
    fn eq(&self, other: &Zqf) -> bool {
        self.get_element() == other.get_element()
    }
}

impl ECPoint for Zqg{
    type SecretKey = BigInt;
    type PublicKey = BigInt;
    type Scalar = Zqf;

    fn base_point2() -> Self {
        unimplemented!()
    }

    fn generator() -> Self {
        unimplemented!()
    }

    fn get_element(&self) -> Self::PublicKey {
        unimplemented!()
    }

    fn x_coor(&self) -> Option<BigInt> {
        unimplemented!()
    }

    fn y_coor(&self) -> Option<BigInt> {
        unimplemented!()
    }

    fn bytes_compressed_to_big_int(&self) -> BigInt {
        unimplemented!()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorKey> {
        unimplemented!()
    }

    fn pk_to_key_slice(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn scalar_mul(&self, fe: &Self::SecretKey) -> Self {
        unimplemented!()
    }

    fn add_point(&self, other: &Self::PublicKey) -> Self {
        unimplemented!()
    }

    fn sub_point(&self, other: &Self::PublicKey) -> Self {
        unimplemented!()
    }

    fn from_coor(x: &BigInt, y: &BigInt) -> Self {
        unimplemented!()
    }
}

impl Mul<Zqf> for Zqg {
    type Output = Zqg;
    fn mul(self, other: Zqf) -> Zqg {
        self.scalar_mul(&other.get_element())
    }
}

impl<'o> Mul<&'o Zqf> for Zqg {
    type Output = Zqg;
    fn mul(self, other: &'o Zqf) -> Zqg {
        self.scalar_mul(&other.get_element())
    }
}

impl<'o> Mul<&'o Zqf> for &'o Zqg {
    type Output = Zqg;
    fn mul(self, other: &'o Zqf) -> Zqg {
        self.scalar_mul(&other.get_element())
    }
}

impl Add<Zqg> for Zqg {
    type Output = Zqg;
    fn add(self, other: Zqg) -> Zqg {
        self.add_point(&other.get_element())
    }
}

impl<'o> Add<&'o Zqg> for Zqg {
    type Output = Zqg;
    fn add(self, other: &'o Zqg) -> Zqg {
        self.add_point(&other.get_element())
    }
}

impl<'o> Add<&'o Zqg> for &'o Zqg {
    type Output = Zqg;
    fn add(self, other: &'o Zqg) -> Zqg {
        self.add_point(&other.get_element())
    }
}
impl PartialEq for Zqg {
    fn eq(&self, other: &Zqg) -> bool {
        self.get_element() == other.get_element()
    }
}