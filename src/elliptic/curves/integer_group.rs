use crate::{BigInt, ErrorKey};
use std::ops::{Add, Mul, Sub};
use crate::elliptic::curves::traits::{ECScalar, ECPoint};
use crate::arithmetic::traits::Samplable;
use std::borrow::Cow;

#[derive(Clone)]
pub struct Zqg {
    g: BigInt,
}

#[derive(Clone)]
pub struct Zqf {
    f: BigInt,
}
lazy_static::lazy_static! {
pub static ref Q:BigInt = {
    let mut lbslice: [u8; 256] = [0xff as u8; 256];
    lbslice[0] = 0x7f;
    let lb = BigInt::from(&lbslice[..]);
    lb.nextprime()
   };
}
pub type GE = Zqg;
pub type FE = Zqf;

impl ECScalar for Zqf {
    type SecretKey = Cow<'static, Zqf>;

    fn new_random() -> Self {
        Zqf { f: BigInt::sample_below(&FE::q()) }
    }

    fn zero() -> Self {
        Zqf { f: BigInt::zero() }
    }

    fn get_element(&'static self) -> Self::SecretKey {
        Cow::Borrowed(self)
    }

    fn set_element(&mut self, element: Self::SecretKey) {
        self.f = element.into_owned().f;
    }

    fn from(n: &BigInt) -> Self {
        Zqf { f: n.clone() }
    }

    fn to_big_int(&self) -> BigInt {
        self.get_element().into_owned().f
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
        self.f.mul(other.f);
        self
    }
}

impl<'o> Mul<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn mul(self, other: &'o Zqf) -> Zqf {
        self.f.mul(other.to_big_int());
        self
    }
}

impl Add<Zqf> for Zqf {
    type Output = Zqf;
    fn add(self, other: Zqf) -> Zqf {
        self.f.add(other.f);
        self
    }
}

impl<'o> Add<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn add(self, other: &'o Zqf) -> Zqf {
        self.f.add(other.to_big_int());
        self
    }
}

impl PartialEq for Zqf {
    fn eq(&self, other: &Zqf) -> bool {
        self.get_element() == other.get_element()
    }
}

impl ECPoint for Zqg {
    type SecretKey = Cow<'static, Zqf>;
    type PublicKey = Cow<'static, Zqg>;
    type Scalar = Zqf;

    fn base_point2() -> Self {
        unimplemented!()
    }

    fn generator() -> Self {
        Zqg { g: BigInt::one() }
    }

    fn get_element(&'static self) -> Self::PublicKey {
        Cow::Borrowed(self)
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
        Zqg { g: *self.g * fe.to_big_int() }
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
        self.scalar_mul(&Cow::Owned(other))
    }
}

impl<'o> Mul<&'o Zqf> for Zqg {
    type Output = Zqg;
    fn mul(self, other: &'o Zqf) -> Zqg {
        self.scalar_mul(&Cow::Borrowed(other))
    }
}

impl<'o> Mul<&'o Zqf> for &'o Zqg {
    type Output = Zqg;
    fn mul(self, other: &'o Zqf) -> Zqg {
        self.scalar_mul(&Cow::Borrowed(other))
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

#[cfg(test)]
mod tests {
    use crate::BigInt;

    #[test]
    fn test_zqf_mul() {
        let zqf1 = BigInt::from(99u64);
        let zqf2 = BigInt::from(100u64);
        let zqf3 = zqf1 * zqf2;
        assert!(zqf3, BigInt::from(9900u64));
        //println!("{}",zqf1)
    }
}