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

        cs.enforce(
            || "input equals output",
            |lc| lc + total_input.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + total_output.get_variable(),
        );

        Ok(())
    }
}
fn create_rollup_proof(transactions: Vec<Transaction>) -> bool {
    let total_input: u32 = transactions.iter().map(|tx| tx.sender).sum();
    let total_output: u32 = transactions.iter().map(|tx| tx.receiver).sum();

    let total_input_fr = Fr::from_str(&total_input.to_string()).unwrap();
    let total_output_fr = Fr::from_str(&total_output.to_string()).unwrap();

    let circuit = RollupCircuit {
        transactions,
        total_input: Some(total_input_fr),
        total_output: Some(total_output_fr),
    };

    let rng = &mut thread_rng();
    let params = generate_random_parameters::<Bn256, _, _>(circuit.clone(), rng).expect("Failed to generate parameters");

    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, rng).expect("Failed to create proof");

    let public_input = vec![total_input_fr, total_output_fr];
    verify_proof(&pvk, &proof, &public_input).is_ok()
}

fn main() {
    let transactions = vec![
        Transaction { sender: 10, receiver: 5, amount: 5, sender_balance: 10 }, 
          
    ];

    let is_valid = create_rollup_proof(transactions);

    println!("zk-Rollup proof valid: {}", is_valid);
}


