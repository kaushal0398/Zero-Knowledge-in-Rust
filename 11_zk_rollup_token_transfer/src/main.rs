use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::{Bn256, Fr};
use bellman::groth16::{generate_random_parameters, 
use rand::thread_rng;
use bellman::gadgets::num::AllocatedNum;

#[derive(Clone)]
struct Transaction {
    sender_id: u32,    
    receiver_id: u32,  
    amount: u32,       
}

struct RollupCircuit {
    transactions: Vec<Transaction>,  
    user_balances: Vec<u32>,        
}

impl Circuit<Fr> for RollupCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let num_users = self.user_balances.len();

        if self.transactions.iter().any(|tx| tx.sender_id as usize >= num_users || tx.receiver_id as usize >= num_users) {
            return Err(SynthesisError::Unsatisfiable);
        }

        let mut allocated_balances = Vec::with_capacity(num_users);
        for (i, balance) in self.user_balances.iter().enumerate() {
            let balance_alloc = AllocatedNum::alloc(cs.namespace(|| format!("user balance {}", i)), || {
                Ok(Fr::from_str(&balance.to_string()).unwrap())
            })?;
            allocated_balances.push(balance_alloc);
        }

        for (i, tx) in self.transactions.iter().enumerate() {
            let sender_balance = &allocated_balances[tx.sender_id as usize];
            let receiver_balance = &allocated_balances[tx.receiver_id as usize];

            let amount = AllocatedNum::alloc(cs.namespace(|| format!("transaction amount {}", i)), || {
                Ok(Fr::from_str(&tx.amount.to_string()).unwrap())
            })?;

            cs.enforce(
                || format!("balance check for sender {}", i),
                |lc| lc + sender_balance.get_variable(),
                |lc| lc + CS::one(),
                |lc| lc + amount.get_variable(),
            );

            let new_sender_balance = AllocatedNum::alloc(cs.namespace(|| format!("new sender balance {}", i)), || {
                let mut sb = Fr::from_str(&self.user_balances[tx.sender_id as usize].to_string()).unwrap();
                sb.sub_assign(&Fr::from_str(&tx.amount.to_string()).unwrap());
                if sb.is_zero() {
                    return Err(SynthesisError::Unsatisfiable);
                }
                Ok(sb)
            })?;

            cs.enforce(
                || format!("update sender balance {}", i),
                |lc| lc + sender_balance.get_variable(),
                |lc| lc - amount.get_variable(),
                |lc| lc + new_sender_balance.get_variable(),
            );

            let new_receiver_balance = AllocatedNum::alloc(cs.namespace(|| format!("new receiver balance {}", i)), || {
                let mut rb = Fr::from_str(&self.user_balances[tx.receiver_id as usize].to_string()).unwrap();
                rb.add_assign(&Fr::from_str(&tx.amount.to_string()).unwrap());
                Ok(rb)
            })?;

            cs.enforce(
                || format!("update receiver balance {}", i),
                |lc| lc + receiver_balance.get_variable(),
                |lc| lc + amount.get_variable(),
                |lc| lc + new_receiver_balance.get_variable(),
            );
        }

        let total_input: u32 = self.transactions.iter().map(|tx| tx.amount).sum();
        let total_output: u32 = total_input;
        let total_input_fr = Fr::from_str(&total_input.to_string()).unwrap();
        let total_output_fr = Fr::from_str(&total_output.to_string()).unwrap();

        let total_input_alloc = AllocatedNum::alloc(cs.namespace(|| "total input"), || Ok(total_input_fr))?;
        let total_output_alloc = AllocatedNum::alloc(cs.namespace(|| "total output"), || Ok(total_output_fr))?;

        cs.enforce(
            || "total input equals total output",
            |lc| lc + total_input_alloc.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + total_output_alloc.get_variable(),
        );

        Ok(())
    }
}

fn create_rollup_proof(transactions: Vec<Transaction>, user_balances: Vec<u32>) -> bool {
    let circuit = RollupCircuit {
        transactions,
        user_balances,
    };

    let rng = &mut thread_rng();
    let params = generate_random_parameters::<Bn256, _, _>(circuit.clone(), rng).expect("Failed to generate parameters");

    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, rng).expect("Failed to create proof");

    let public_input = vec![Fr::from_str("0").unwrap()]; 
    verify_proof(&pvk, &proof, &public_input).is_ok()
}

fn main() {
    let transactions = vec![
        Transaction { sender_id: 0, receiver_id: 1, amount: 5 },  
        Transaction { sender_id: 2, receiver_id: 0, amount: 8 },  
    ];

    let user_balances = vec![10, 5, 8];  

    let is_valid = create_rollup_proof(transactions, user_balances);

    println!("zk-Rollup proof valid: {}", is_valid);
}

