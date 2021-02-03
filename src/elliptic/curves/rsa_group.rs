use crate::arithmetic::traits::{Converter, Modulo, Samplable};
use crate::{BigInt};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
use std::ptr;

#[derive(Clone, Debug)]
pub struct Zqf {
    f: BigInt,
}
lazy_static::lazy_static! {
pub static ref p:BigInt= {
    let mut lbslice: [u8; 8] = [0xff as u8; 8];
    lbslice[0] = 0x7f;
    let modlus = BigInt::from(4);
    let target = BigInt::from(3);
    let mut lb = BigInt::from(&lbslice[..]);
    lb = lb.nextprime();
    while (lb.mod_floor(&modlus) != target){
    lb = lb.nextprime();
    }
    lb
   };
   pub static ref q:BigInt = {
    let modlus = BigInt::from(4);
    let target = BigInt::from(3);
    let mut lb = p.nextprime();
    while (lb.mod_floor(&modlus) != target){
    lb = lb.nextprime();
    }
    lb
   };
   pub static ref M:BigInt ={
      p.clone()*q.clone()
   };
   pub static ref Phi:BigInt ={
      (p.clone()-1)*(q.clone()-1)
   };
}

impl Zqf {
    fn new_random() -> Self {
        Zqf {
            f: BigInt::sample_below(&M),
        }
    }

    fn zero() -> Self {
        Zqf { f: BigInt::zero() }
    }

    fn get_element(&self) -> Self {
        self.clone()
    }

    fn set_element(&mut self, element: Self) {
        self.f = element.f;
    }

    fn from(n: &BigInt) -> Self {
        Zqf { f: n.clone() }
    }

    fn to_big_int(&self) -> BigInt {
        self.get_element().f
    }

    fn q() -> BigInt {
        Phi.clone()
    }

    fn pow(self, e: &BigInt) -> Self {
        Zqf {
            f: BigInt::mod_pow(&self.f, e, &M),
        }
    }

    fn invert(&self) -> Self {
        Zqf {
            f: self.f.invert(&M).unwrap(),
        }
    }
}

impl Mul<Zqf> for Zqf {
    type Output = Zqf;
    fn mul(mut self, other: Zqf) -> Zqf {
        self.f = BigInt::mod_mul(&self.f, &other.f, &M);
        self
    }
}

impl<'o> Mul<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn mul(mut self, other: &'o Zqf) -> Zqf {
        self.f = BigInt::mod_mul(&self.f, &other.f, &M);
        self
    }
}

impl Add<Zqf> for Zqf {
    type Output = Zqf;
    fn add(mut self, other: Zqf) -> Zqf {
        self.f = BigInt::mod_add(&self.f, &other.f, &M);
        self
    }
}

impl<'o> Add<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn add(mut self, other: &'o Zqf) -> Zqf {
        self.f = BigInt::mod_add(&self.f, &other.f, &M);
        self
    }
}

impl PartialEq for Zqf {
    fn eq(&self, other: &Zqf) -> bool {
        self.f == other.f
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptographic_primitives::hashing::hash_sha256::HSha256;
    use crate::cryptographic_primitives::hashing::traits::Hash;
    use crate::BigInt;

    #[test]
    fn test_pow() {
        let zqf1 = BigInt::from(99u64);
        let zqf2 = BigInt::from(100u64);
        //let zqf3 = zqf1 * zqf2;
        let u = Zqf { f: zqf1 };
        let v = u.pow(&zqf2);
        println!("{:?}", v);
        //assert_eq!(zqf3, BigInt::from(9900u64));
    }
}
