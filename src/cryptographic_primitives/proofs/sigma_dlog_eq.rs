/*
    This file is part of Curv library
    Copyright 2018 by Kzen Networks
    (https://github.com/KZen-networks/curv)
    License MIT: https://github.com/KZen-networks/curv/blob/master/LICENSE
*/

use super::ProofError;

use crate::elliptic::curves::traits::*;

use crate::cryptographic_primitives::hashing::hash_sha256::HSha256;
use crate::cryptographic_primitives::hashing::traits::Hash;
use zeroize::Zeroize;

/// This is implementation of Schnorr's identification protocol for elliptic curve groups or a
/// sigma protocol for Proof of knowledge of the discrete log of an Elliptic-curve point:
/// C.P. Schnorr. Efficient Identification and Signatures for Smart Cards. In
/// CRYPTO 1989, Springer (LNCS 435), pages 239–252, 1990.
/// https://pdfs.semanticscholar.org/8d69/c06d48b618a090dd19185aea7a13def894a5.pdf.
///
/// The protocol is using Fiat-Shamir Transform: Amos Fiat and Adi Shamir.
/// How to prove yourself: Practical solutions to identification and signature problems.
/// In Advances in Cryptology - CRYPTO ’86, Santa Barbara, California, USA, 1986, Proceedings,
/// pages 186–194, 1986.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DLogEqProof<P: ECPoint> {
    pub generator: (P, P),
    pub pk: (P, P),
    pub pk_t_rand_commitment: (P, P),
    pub challenge_response: P::Scalar,
}

impl<P> DLogEqProof<P>
    where
        P: ECPoint + Clone,
        P::Scalar: Zeroize,
{
    pub fn prove(sk: &P::Scalar, g0: &P, g1: &P) -> DLogEqProof<P> {
        //g_0 g_1 in the form of BigInt
        let generator_0 = g0.bytes_compressed_to_big_int();
        let generator_1 = g1.bytes_compressed_to_big_int();

        //u from Z_n
        let mut sk_t_rand_commitment: P::Scalar = ECScalar::new_random();

        //a_0=g_0^(u) a_1=g_1^(u)
        let pk_t_rand_commitment_0 = g0.scalar_mul(&sk_t_rand_commitment.get_element());
        let pk_t_rand_commitment_1 = g1.scalar_mul(&sk_t_rand_commitment.get_element());

        let pk_0 = g0.scalar_mul(&sk.get_element());
        let pk_1 = g1.scalar_mul(&sk.get_element());

        let challenge = HSha256::create_hash(&[
            &pk_t_rand_commitment_0.bytes_compressed_to_big_int(),
            &pk_t_rand_commitment_1.bytes_compressed_to_big_int(),
            &generator_0,
            &generator_1,
            &pk_0.bytes_compressed_to_big_int(),
            &pk_1.bytes_compressed_to_big_int(),
        ]);
        //c = hash(g_0,g_1,h_0,h_1,a_0,a_1)
        let challenge_fe: P::Scalar = ECScalar::from(&challenge);
        //c*x
        let challenge_mul_sk = challenge_fe.mul(&sk.get_element());
        //u-c*x
        let challenge_response = sk_t_rand_commitment.sub(&challenge_mul_sk.get_element());
        sk_t_rand_commitment.zeroize();

        DLogEqProof {
            generator: (g0.clone(), g1.clone()),
            pk: (pk_0, pk_1),
            pk_t_rand_commitment: (pk_t_rand_commitment_0, pk_t_rand_commitment_1),
            challenge_response,
        }
    }

    pub fn verify(proof: &DLogEqProof<P>) -> Result<(), ProofError> {
        let challenge = HSha256::create_hash(&[
            &proof.pk_t_rand_commitment.0.bytes_compressed_to_big_int(),
            &proof.pk_t_rand_commitment.1.bytes_compressed_to_big_int(),
            &proof.generator.0.bytes_compressed_to_big_int(),
            &proof.generator.1.bytes_compressed_to_big_int(),
            &proof.pk.0.bytes_compressed_to_big_int(),
            &proof.pk.1.bytes_compressed_to_big_int(),
        ]);
        //c
        let sk_challenge: P::Scalar = ECScalar::from(&challenge);
        //h_0; h_1;
        let pk0 = proof.pk.0.clone();
        let pk1 = proof.pk.1.clone();
        //h_0^c h_1^c
        let pk_challenge_0 = pk0.scalar_mul(&sk_challenge.get_element());
        let pk_challenge_1 = pk1.scalar_mul(&sk_challenge.get_element());

        let mut pk_verifier_0 = proof.generator.0.scalar_mul(&proof.challenge_response.get_element());
        let mut pk_verifier_1 = proof.generator.1.scalar_mul(&proof.challenge_response.get_element());

        pk_verifier_0 = pk_verifier_0.add_point(&pk_challenge_0.get_element());
        pk_verifier_1 = pk_verifier_1.add_point(&pk_challenge_1.get_element());

        if (pk_verifier_0, pk_verifier_1) == proof.pk_t_rand_commitment {
            Ok(())
        } else {
            Err(ProofError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    crate::test_for_all_curves!(test_dlog_proof);
    fn test_dlog_proof<P>()
        where
            P: ECPoint + Clone,
            P::Scalar: Zeroize,
    {
        let witness: P::Scalar = ECScalar::new_random();
        let g0 =P::generator();
        let g1=P::base_point2();
        let dlog_proof = DLogEqProof::<P>::prove(&witness,&g0,&g1);
        let verified = DLogEqProof::verify(&dlog_proof);
        match verified {
            Ok(_t) => assert!(true),
            Err(_e) => assert!(false),
        }
    }
}
