use crate::arithmetic::traits::{Converter, Modulo, Samplable};
use crate::{BigInt};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, Div, DivAssign};
use std::ptr;
use std::borrow::Borrow;

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
   pub static ref g : BigInt ={
       Zqf::get_element_order_without_small_divisor().f
   };
   pub static ref alpha : BigInt ={
       Zqf::get_random_from_z_phi().f
   };
}

impl From<BigInt> for Zqf{
    fn from(n: BigInt) -> Self {
        Zqf{
            f:n,
        }
    }
}
impl From<&BigInt> for Zqf{
    fn from(n: &BigInt) -> Self {
        Zqf{
            f:n.clone(),
        }
    }
}
// Conflict with clone implementation
// impl ToOwned for Zqf{
//     type Owned = BigInt;
//     fn to_owned(&self) -> Self::Owned {
//         self.f.clone()
//     }
// }
impl AsRef<BigInt> for Zqf{
    fn as_ref(&self) -> &BigInt {
        &self.f
    }
}
impl Into<BigInt> for Zqf{
    fn into(self) -> BigInt {
        self.f
    }
}
impl Zqf {
    pub fn to_big_int(&self) -> BigInt {
        self.f.clone()
    }
    fn get_divisor_exp_in_phi(divisor: &BigInt) -> u32 {
        let mut uint0 = Phi.clone();
        let mut i = 0;
        while uint0.is_multiple_of(divisor) {
            uint0 = uint0.div(divisor);
            i += 1;
        }
        return i;
    }
    fn get_element_order_without_small_divisor() -> Self {
        let f = BigInt::sample_below(&M.clone().sub(1));
        let mut one = BigInt::from(1);
        let mut exp = BigInt::one();
        while one < BigInt::from(128) {
            one = one.nextprime();
            exp *= one.pow(Self::get_divisor_exp_in_phi(&one));
        }
        Zqf { f: f.powm(&exp, &M) }
    }

    fn get_random_from_z_phi() -> Self {
        Zqf {
            f: BigInt::sample_below(&Phi.clone().sub(1)),
        }
    }

    pub fn zero() -> Self {
        Zqf { f: BigInt::zero() }
    }

    fn q() -> BigInt {
        Phi.clone()
    }

    pub fn pow_mod_m(&self, e: &BigInt) -> Self {
        Zqf {
            f: BigInt::mod_pow(&self.f, e, &M),
        }
    }

    pub fn pow_mod_phi(&self, e: &BigInt) -> Self {
        Zqf {
            f: BigInt::mod_pow(&self.f, e, &Phi),
        }
    }

    pub fn sqrt(&self) -> Self {
        Zqf{
            f: BigInt::root(&self.f,2).mod_floor(&M),
        }
    }

    pub fn square(&self) -> Self{
        let pow = BigInt::from(2);
        self.pow_mod_m(pow.borrow())
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
        let v = u.pow_mod_m(&zqf2);
        println!("{:?}", v);

        let gg = Zqf::get_element_order_without_small_divisor();
        println!("{:?}", &Phi.clone());
        println!("{:?}", gg);

        let usize_length = M.bit_length();
        println!("The usize_length is :{:?}...",usize_length);
        //assert_eq!(zqf3, BigInt::from(9900u64));
    }

    #[test]
    fn test_group_order(){
        let generator = Zqf::from(g.clone());
        let one = generator.pow_mod_m(Phi.borrow());
        let one_b = BigInt::from(1);
        assert_eq!(one.to_big_int(),one_b)
    }

    #[test]
    fn test_bigint_sqrt(){
        let a = BigInt::from(2);
        let b = BigInt::from(4);
        assert_eq!(a,b.sqrt())
    }

    #[test]
    fn test_sqrt(){
        let k = 10;
        let generator = Zqf::from(g.clone());

        let base = Zqf::from(&BigInt::from(2));
        let exp0 = BigInt::from(2_u64.pow(k - 1));
        let exp1 = BigInt::from(2_u64.pow(k));
        let exp1_0 = BigInt::from(2_u64.pow(k)-2);
        let exp1_1 = BigInt::from(2_u64.pow(k)-1);
        let a_0 = base.pow_mod_phi(&exp0);
        let a_1 = base.pow_mod_phi(&exp1);
        let a_1_0 = base.pow_mod_phi(&exp1_0);
        let a_1_1 = base.pow_mod_phi(&exp1_1);

        let u_0 = generator.pow_mod_m(a_0.as_ref());
        let u_1 = generator.pow_mod_m(a_1.as_ref());

        let m_1 = generator.pow_mod_m(a_1_1.as_ref());

        let m_0 = generator.pow_mod_m(a_1_0.as_ref());

        assert_eq!(m_0.square().to_big_int(),m_1.to_big_int());
        assert_eq!(m_1.square().to_big_int(),u_1.to_big_int());
    }
}
