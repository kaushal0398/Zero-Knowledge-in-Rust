use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::Fr;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
pub fn pedersen_commit(value: u64, blinding: Scalar) -> RistrettoPoint {
    let value_scalar = Scalar::from(value);
    value_scalar * RISTRETTO_BASEPOINT_POINT + blinding * RISTRETTO_BASEPOINT_POINT
}

pub struct CommitmentCircuit {
    pub value: Option<Fr>,
    pub blinding: Option<Fr>,
}

impl Circuit<Fr> for CommitmentCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let value = AllocatedNum::alloc(cs.namespace(|| "value"), || self.value.ok_or(SynthesisError::AssignmentMissing))?;
        let blinding = AllocatedNum::alloc(cs.namespace(|| "blinding"), || self.blinding.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce(|| "Pedersen commitment", |lc| lc + value.get_variable() + blinding.get_variable(), |lc| lc + CS::one(), |lc| lc + CS::one());

        Ok(())
    }
}
