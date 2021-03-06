use crate::arithmetic::traits::{Converter, Modulo, Samplable};
use crate::elliptic::curves::traits::{ECPoint, ECScalar};
use crate::{BigInt, ErrorKey};
use super::rsa_group;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub};
use std::ptr;
use std::sync::atomic;
use zeroize::Zeroize;

#[derive(Clone, Debug)]
pub struct Zqg {
    g: BigInt,
}

#[derive(Clone, Debug)]
pub struct Zqf {
    f: BigInt,
}
//2048bit
// let mut lbslice: [u8; 256] = [0xff as u8; 256];
// lbslice[0] = 0x7f;

//128bit 64bit*64bit
// let mut lbslice: [u8; 8] = [0xff as u8; 8];
// lbslice[0] = 0x7f;
lazy_static::lazy_static! {
   pub static ref Q:BigInt = {
      rsa_group::M.clone()
   };
   // {
   //  let mut lbslice: [u8; 256] = [0xff as u8; 256];
   //  lbslice[0] = 0x7f;
   //  let mut lb = BigInt::from(&lbslice[..]);
   //  lb = lb.nextprime();
   //  lb
   // };
}
pub type GE = Zqg;
pub type FE = Zqf;
impl From<&BigInt> for Zqf {
    fn from(n: &BigInt) -> Self {
        <Self as ECScalar>::from(n)
    }
}
impl ECScalar for Zqf {
    type SecretKey = Zqf;

    fn new_random() -> Self {
        Zqf {
            f: BigInt::sample_below(&FE::q()),
        }
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

    fn q() -> BigInt {
        rsa_group::Phi.clone()
    }

    fn add(&self, other: &Self::SecretKey) -> Self {
        self.clone().add(other)
    }

    fn mul(&self, other: &Self::SecretKey) -> Self {
        self.clone().mul(other)
    }

    fn sub(&self, other: &Self::SecretKey) -> Self {
        Zqf {
            f: BigInt::mod_sub(&self.f, &other.f, &Self::q()),
        }
    }

    fn invert(&self) -> Self {
        Zqf {
            f: self.f.invert(&Self::q()).unwrap(),
        }
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
    fn q() -> BigInt {
        Q.clone()
    }
}
impl From<&BigInt> for Zqg {
    fn from(n: &BigInt) -> Self {
        Zqg{
            g: n.clone()
        }
    }
}
impl From<BigInt> for Zqg {
    fn from(n: BigInt) -> Self {
        Zqg{
            g: n
        }
    }
}
impl ECPoint for Zqg {
    type SecretKey = Zqf;
    type PublicKey = Zqg;
    type Scalar = Zqf;

    fn base_point2() -> Self {
        Zqg {
            g: BigInt::from(1378),
        }
    }

    fn generator() -> Self {
        Zqg { g: rsa_group::g.clone() }
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
        Ok(Zqg {
            g: BigInt::from(bytes),
        })
    }

    fn pk_to_key_slice(&self) -> Vec<u8> {
        let mut hex_str = self.g.to_hex();
        if (hex_str.len() % 2 != 0) {
            unsafe {
                hex_str.as_mut_vec().insert(0, b'0');
            }
        }
        hex::decode(hex_str).unwrap().to_vec()
        //hex_str
    }

    fn scalar_mul(&self, fe: &Self::SecretKey) -> Self {
        Zqg {
            g: BigInt::mod_pow(&self.g, &fe.f, &Self::q()),
        }
    }

    fn add_point(&self, other: &Self::PublicKey) -> Self {
        Zqg {
            g: BigInt::mod_mul(&self.g, &other.g, &Self::q()),
        }
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
    fn mul(mut self, other: Zqf) -> Zqg {
        self.g = BigInt::mod_pow(&self.g, &other.f, &Self::q());
        self
    }
}

impl<'o> Mul<&'o Zqf> for Zqg {
    type Output = Zqg;
    fn mul(mut self, other: &'o Zqf) -> Zqg {
        self.g = BigInt::mod_pow(&self.g, &other.f, &Self::q());
        self
    }
}

impl Add<Zqg> for Zqg {
    type Output = Zqg;
    fn add(mut self, other: Zqg) -> Zqg {
        self.g = BigInt::mod_mul(&self.g, &other.g, &Self::q());
        self
    }
}

impl<'o> Add<&'o Zqg> for Zqg {
    type Output = Zqg;
    fn add(mut self, other: &'o Zqg) -> Zqg {
        self.g = BigInt::mod_mul(&self.g, &other.g, &Self::q());
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
    use super::*;
    use crate::cryptographic_primitives::hashing::hash_sha256::HSha256;
    use crate::cryptographic_primitives::hashing::traits::Hash;
    use crate::BigInt;
    use std::borrow::Borrow;

    #[test]
    fn test_zqf_mul() {
        let zqf1 = BigInt::from(99u64);
        let zqf2 = BigInt::from(100u64);
        let zqf3 = zqf1 * zqf2;
        assert_eq!(zqf3, BigInt::from(9900u64));
        println!("{:?}", Zqf::q())
    }

    #[test]
    fn test_zqg_from() {
        let zqg = Zqg {
            g: BigInt::from(1200u64),
        };
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

    #[test]
    fn test_zqg_add(){
        let a_inner = BigInt::from(2);
        let a = Zqg::from(a_inner);
        let b_inner = BigInt::from(4);
        let b = Zqg::from(b_inner);
        let c = a + b;

        let c_0 = BigInt::from(8);
        assert_eq!(c,Zqg::from(c_0))
    }
    #[test]
    fn test_zqg_mul_scalar(){
        let a_inner = BigInt::from(2).pow(50);
        let a:Zqf = a_inner.borrow().into();
        let b_inner = BigInt::from(7).pow(39);
        let b:Zqf = b_inner.borrow().into();
        let d_inner = BigInt::from(13);
        let d:Zqf = d_inner.borrow().into();
        let c = a * b + d;

        let base = Zqg::generator();

        let base_c = base.borrow() * c.borrow();

        let a_0:Zqf = a_inner.borrow().into();
        let b_0:Zqf = b_inner.borrow().into();
        let d_0:Zqf = d_inner.borrow().into();

        let base_left = base.borrow() * a_0.borrow();

        let base_ab = base_left * b_0 + base * d_0;

        assert_eq!(base_c,base_ab)
    }
}
