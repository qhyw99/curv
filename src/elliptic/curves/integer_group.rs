use crate::{BigInt, ErrorKey};
use std::ops::{Add, Mul, Sub, MulAssign};
use crate::elliptic::curves::traits::{ECScalar, ECPoint};
use p256::Scalar;
use crate::arithmetic::traits::Samplable;

#[macro_use]
extern crate lazy_static;
lazy_static! {
pub static ref Q:BigInt = {
    let mut lbslice: [u8; 256] = [0xff as u8; 256];
    lbslice[0] = 0x7f;
    let lb = BigInt::from(&lbslice[..]);
    lb.nextprime()
   };
}
#[derive(Clone, Copy)]
pub struct Zqg {
    g: BigInt,
}

#[derive(Clone, Copy)]
pub struct Zqf {
    f: BigInt,
}

pub type GE = Zqg;
pub type FE = Zqf;

impl ECScalar for Zqf {
    type SecretKey = BigInt;

    fn new_random() -> Self {
        Zqf { f: BigInt::sample_below(&FE::q()) }
    }

    fn zero() -> Self {
        Zqf { f: BigInt::zero() }
    }

    fn get_element(&self) -> Self::SecretKey {
        *self.f
    }

    fn set_element(&mut self, element: Self::SecretKey) {
        self.f = element
    }

    fn from(n: &BigInt) -> Self {
        Zqf { f: n.clone() }
    }

    fn to_big_int(&self) -> BigInt {
        self.get_element()
    }

    fn q() -> BigInt { *Q }

    fn add(&self, other: &Self::SecretKey) -> Self {
        Zqf { f: self.f.add(other) }
    }

    fn mul(&self, other: &Self::SecretKey) -> Self {
        Zqf { f: self.f.mul(other) }
    }

    fn sub(&self, other: &Self::SecretKey) -> Self {
        Zqf { f: self.f.sub(other) }
    }

    fn invert(&self) -> Self {
        Zqf { f: self.f.invert(&Self::q()).unwrap() }
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

impl ECPoint for Zqg {
    type SecretKey = BigInt;
    type PublicKey = BigInt;
    type Scalar = Zqf;

    fn base_point2() -> Self {
        unimplemented!()
    }

    fn generator() -> Self {
        Zqg { g: BigInt::one() }
    }

    fn get_element(&self) -> Self::PublicKey {
        *self.g
    }

    fn x_coor(&self) -> Option<BigInt> {
        Some(*self.g)
    }

    fn y_coor(&self) -> Option<BigInt> {
        Some(*self.g)
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
        Zqg { g: *self.g * fe }
    }

    fn add_point(&self, other: &Self::PublicKey) -> Self {
        Zqg { g: self.g.add(other) }
    }

    fn sub_point(&self, other: &Self::PublicKey) -> Self {
        Zqg { g: self.g.sub(other) }
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