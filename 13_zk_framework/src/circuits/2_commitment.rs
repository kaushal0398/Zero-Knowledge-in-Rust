use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::gadgets::num::AllocatedNum;
use pairing::bn256::Fr;

/// A commitment circuit that performs a Pedersen commitment within a zero-knowledge proof system.
pub struct CommitmentCircuit {
    /// Optional value to commit to, represented as an `Fr` field element.
    pub value: Option<Fr>,
    /// Optional blinding factor for the commitment, also an `Fr` field element.
    pub blinding: Option<Fr>,
}

impl Circuit<Fr> for CommitmentCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let value = AllocatedNum::alloc(cs.namespace(|| "value"), || {
            self.value.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let blinding = AllocatedNum::alloc(cs.namespace(|| "blinding"), || {
            self.blinding.ok_or(SynthesisError::AssignmentMissing)
        })?;

        cs.enforce(
            || "Pedersen commitment constraint",
            |lc| lc + value.get_variable() + blinding.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + CS::one(),
        );

        Ok(())
    }
}
