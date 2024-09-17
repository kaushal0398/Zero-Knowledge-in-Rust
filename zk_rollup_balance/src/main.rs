use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::{Bn256, Fr};
use bellman::groth16::{generate_random_parameters, create_random_proof, verify_proof, prepare_verifying_key};
use rand::thread_rng;
use bellman::gadgets::num::AllocatedNum;

#[derive(Clone)]
struct Transaction {
    sender: u32,
    receiver: u32,
    amount: u32,
    sender_balance: u32, 
}

struct RollupCircuit {
    transactions: Vec<Transaction>,  
    total_input: Option<Fr>,        
    total_output: Option<Fr>,        
}

impl Circuit<Fr> for RollupCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let total_input = AllocatedNum::alloc(cs.namespace(|| "total input"), || {
            self.total_input.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let total_output = AllocatedNum::alloc(cs.namespace(|| "total output"), || {
            self.total_output.ok_or(SynthesisError::AssignmentMissing)
        })?;

        for (i, tx) in self.transactions.iter().enumerate() {
            let sender_balance = AllocatedNum::alloc(cs.namespace(|| format!("sender balance {}", i)), || {
                Ok(Fr::from_str(&tx.sender_balance.to_string()).unwrap())
            })?;
            let amount = AllocatedNum::alloc(cs.namespace(|| format!("transaction amount {}", i)), || {
                Ok(Fr::from_str(&tx.amount.to_string()).unwrap())
            })?;

            cs.enforce(
                || format!("balance check {}", i),
                |lc| lc + sender_balance.get_variable(),
                |lc| lc + CS::one(),
                |lc| lc + amount.get_variable()
            );
        }

        