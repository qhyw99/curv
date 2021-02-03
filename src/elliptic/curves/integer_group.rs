use crate::{BigInt, ErrorKey};
use std::ops::{Add, Mul, Sub, MulAssign, AddAssign};
use crate::elliptic::curves::traits::{ECScalar, ECPoint};
use crate::arithmetic::traits::{Samplable, Converter, Modulo};
use zeroize::Zeroize;
use std::sync::atomic;
use std::ptr;

#[derive(Clone, Debug)]
pub struct Zqg {
    g: BigInt,
}

#[derive(Clone, Debug)]
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
    type SecretKey = Zqf;

    fn new_random() -> Self {
        Zqf { f: BigInt::sample_below(&FE::q()) }
    }

    fn zero() -> Self {
        Zqf { f: BigInt::zero() }
    }

    fn get_element(&self) -> Self::SecretKey {
        self.clone()
    }

    fn set_element(&mut self, element: Self::SecretKey) {
        self.f = element.f;
    }

    fn from(n: &BigInt) -> Self {
        Zqf { f: n.clone() }
    }

    fn to_big_int(&self) -> BigInt {
        self.get_element().f
    }

    fn q() -> BigInt { Q.clone() }

    fn add(&self, other: &Self::SecretKey) -> Self {
        self.clone().add(other)
    }

    fn mul(&self, other: &Self::SecretKey) -> Self {
        self.clone().mul(other)
    }

    fn sub(&self, other: &Self::SecretKey) -> Self {
        Zqf { f: BigInt::mod_sub(&self.f, &other.f, &Self::q()) }
    }

    fn invert(&self) -> Self {
        Zqf { f: self.f.invert(&Self::q()).unwrap() }
    }
}

impl Mul<Zqf> for Zqf {
    type Output = Zqf;
    fn mul(mut self, other: Zqf) -> Zqf {
        self.f = BigInt::mod_mul(&self.f, &other.f, &Self::q());
        self
    }
}

impl<'o> Mul<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn mul(mut self, other: &'o Zqf) -> Zqf {
        self.f = BigInt::mod_mul(&self.f, &other.f, &Self::q());
        self
    }
}

impl Add<Zqf> for Zqf {
    type Output = Zqf;
    fn add(mut self, other: Zqf) -> Zqf {
        self.f = BigInt::mod_add(&self.f, &other.f, &Self::q());
        self
    }
}

impl<'o> Add<&'o Zqf> for Zqf {
    type Output = Zqf;
    fn add(mut self, other: &'o Zqf) -> Zqf {
        self.f = BigInt::mod_add(&self.f, &other.f, &Self::q());
        self
    }
}

impl PartialEq for Zqf {
    fn eq(&self, other: &Zqf) -> bool {
        self.f == other.f
    }
}

impl Zeroize for Zqf {
    fn zeroize(&mut self) {
        unsafe { ptr::write_volatile(self, FE::zero()) };
        atomic::fence(atomic::Ordering::SeqCst);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

impl Zqg {
    fn q() -> BigInt { Q.clone() }
}

impl ECPoint for Zqg {
    type SecretKey = Zqf;
    type PublicKey = Zqg;
    type Scalar = Zqf;

    fn base_point2() -> Self {
        Zqg { g: BigInt::from(13) }
    }

    fn generator() -> Self {
        Zqg { g: BigInt::one() }
    }

    fn get_element(&self) -> Self::PublicKey {
        self.clone()
    }

    fn x_coor(&self) -> Option<BigInt> {
        Some(self.clone().g)
    }

    fn y_coor(&self) -> Option<BigInt> {
        Some(self.clone().g)
    }

    fn bytes_compressed_to_big_int(&self) -> BigInt {
        self.g.clone()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, ErrorKey> {
        Ok(Zqg { g: BigInt::from(bytes) })
    }

    fn pk_to_key_slice(&self) -> Vec<u8> {
        let mut hex_str = self.g.to_hex();
        if (hex_str.len() % 2 != 0) {
            unsafe { hex_str.as_mut_vec().insert(0, b'0'); }
        }
        hex::decode(hex_str).unwrap().to_vec()
        //hex_str
    }

    fn scalar_mul(&self, fe: &Self::SecretKey) -> Self {
        Zqg { g: BigInt::mod_mul(&self.g, &fe.f, &Self::q()) }
    }

    fn add_point(&self, other: &Self::PublicKey) -> Self {
        Zqg { g: BigInt::mod_add(&self.g, &other.g, &Self::q()) }
    }

    fn sub_point(&self, other: &Self::PublicKey) -> Self {
        Zqg { g: BigInt::mod_sub(&self.g, &other.g, &Self::q()) }
    }

    fn from_coor(x: &BigInt, y: &BigInt) -> Self {
        unimplemented!()
    }
}

impl Mul<Zqf> for Zqg {
    type Output = Zqg;
    fn mul(mut self, other: Zqf) -> Zqg {
        self.g = BigInt::mod_mul(&self.g, &other.f, &Self::q());
        self
    }
}

impl<'o> Mul<&'o Zqf> for Zqg {
    type Output = Zqg;
    fn mul(mut self, other: &'o Zqf) -> Zqg {
        self.g = BigInt::mod_mul(&self.g, &other.f, &Self::q());
        self
    }
}

impl Add<Zqg> for Zqg {
    type Output = Zqg;
    fn add(mut self, other: Zqg) -> Zqg {
        self.g = BigInt::mod_add(&self.g, &other.g, &Self::q());
        self
    }
}

impl<'o> Add<&'o Zqg> for Zqg {
    type Output = Zqg;
    fn add(mut self, other: &'o Zqg) -> Zqg {
        self.g = BigInt::mod_add(&self.g, &other.g, &Self::q());
        self
    }
}

impl<'o> Mul<&'o Zqf> for &'o Zqg {
    type Output = Zqg;
    fn mul(self, other: &'o Zqf) -> Zqg {
        self.scalar_mul(&other)
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
        self.g == other.g
    }
}

impl Zeroize for Zqg {
    fn zeroize(&mut self) {
        unsafe { ptr::write_volatile(self, GE::generator()) };
        atomic::fence(atomic::Ordering::SeqCst);
        atomic::compiler_fence(atomic::Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;
    use super::*;
    use crate::cryptographic_primitives::hashing::hash_sha256::HSha256;
    use crate::cryptographic_primitives::hashing::traits::Hash;

    #[test]
    fn test_zqf_mul() {
        let zqf1 = BigInt::from(99u64);
        let zqf2 = BigInt::from(100u64);
        let zqf3 = zqf1 * zqf2;
        assert_eq!(zqf3, BigInt::from(9900u64));
        //println!("{}",zqf1)
    }

    #[test]
    fn test_zqg_from() {
        let zqg = Zqg { g: BigInt::from(1200u64) };
        let vr = zqg.pk_to_key_slice();
        println!("{:?}", vr);
        println!("{:?}", BigInt::from(1200u64).to_hex());
        let zqg2 = Zqg::from_bytes(vr.as_slice());
        assert_eq!(zqg2.unwrap().g, BigInt::from(1200u64));
        //zqg2.unwrap().eq(&zqg);
    }

    #[test]
    fn test_zqg_base2() {
        //println!("{:?}",Zqg::base_point2().g);
        let point = Zqg::base_point2();
        //let result1 = HSha256::create_hash_from_ge(&vec![&point, &Zqg::generator()]);
        //assert!(result1.to_big_int().to_str_radix(2).len() > 240);
        let result2 = HSha256::create_hash_from_ge(&vec![&Zqg::generator(), &point]);
        //assert_ne!(result1, result2);
        let result3 = HSha256::create_hash_from_ge(&vec![&Zqg::generator(), &point]);

        println!("{:?} {:?}", result2, result3);
        assert_eq!(result2, result3);
    }
}