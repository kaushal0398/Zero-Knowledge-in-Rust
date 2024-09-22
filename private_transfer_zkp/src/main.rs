use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::{Bn256, Fr};
use bellman::groth16::{generate_random_parameters, create_random_proof, verify_proof, prepare_verifying_key};
use rand::thread_rng;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
fn pedersen_commit(value: u64, blinding: Scalar) -> RistrettoPoint {
    let value_scalar = Scalar::from(value);
    value_scalar * RISTRETTO_BASEPOINT_POINT + blinding * RISTRETTO_BASEPOINT_POINT
}

struct PrivateTransfer {
    sender_balance_commitment: RistrettoPoint,  
    receiver_balance_commitment: RistrettoPoint, 
    transfer_amount_commitment: RistrettoPoint,  
    sender_blinding: Scalar,     
    receiver_blinding: Scalar,  
    transfer_blinding: Scalar,   
    transfer_amount: u64,        
}

impl PrivateTransfer {
    fn new(sender_balance: u64, receiver_balance: u64, transfer_amount: u64) -> Self {
        let sender_blinding = Scalar::random(&mut thread_rng());
        let receiver_blinding = Scalar::random(&mut thread_rng());
        let transfer_blinding = Scalar::random(&mut thread_rng());

        let sender_balance_commitment = pedersen_commit(sender_balance, sender_blinding);
        let receiver_balance_commitment = pedersen_commit(receiver_balance, receiver_blinding);
        let transfer_amount_commitment = pedersen_commit(transfer_amount, transfer_blinding);

        Self {
            sender_balance_commitment,
            receiver_balance_commitment,
            transfer_amount_commitment,
            sender_blinding,
            receiver_blinding,
            transfer_blinding,
            transfer_amount,
        }
    }
}

struct PrivateTransferCircuit {
    sender_balance_commitment: RistrettoPoint,
    receiver_balance_commitment: RistrettoPoint,
    transfer_amount_commitment: RistrettoPoint,
    transfer_amount: Option<Fr>,    
    sender_blinding: Option<Fr>,    
    receiver_blinding: Option<Fr>,  
    transfer_blinding: Option<Fr>,  
}

impl Circuit<Fr> for PrivateTransferCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let transfer_amount = AllocatedNum::alloc(cs.namespace(|| "transfer amount"), || {
            self.transfer_amount.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let sender_blinding = AllocatedNum::alloc(cs.namespace(|| "sender blinding"), || {
            self.sender_blinding.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let receiver_blinding = AllocatedNum::alloc(cs.namespace(|| "receiver blinding"), || {
            self.receiver_blinding.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let transfer_blinding = AllocatedNum::alloc(cs.namespace(|| "transfer blinding"), || {
            self.transfer_blinding.ok_or(SynthesisError::AssignmentMissing)
        })?;

        cs.enforce(
            || "verify transfer commitment",
            |lc| lc + transfer_amount.get_variable() + transfer_blinding.get_variable(),
            |lc| lc + CS::one(),
            |lc| lc + CS::one(), 
        );

        Ok(())
    }
}

/// Function to generate and verify a private transfer proof.
fn generate_private_transfer_proof() -> bool {
    // Set up the transfer.
    let transfer = PrivateTransfer::new(100, 50, 25);

    // Create the ZK-SNARK circuit.
    let circuit = PrivateTransferCircuit {
        sender_balance_commitment: transfer.sender_balance_commitment,
        receiver_balance_commitment: transfer.receiver_balance_commitment,
        transfer_amount_commitment: transfer.transfer_amount_commitment,
        transfer_amount: Some(Fr::from_str(&transfer.transfer_amount.to_string()).unwrap()),
        sender_blinding: Some(Fr::from_str(&transfer.sender_blinding.to_string()).unwrap()),
        receiver_blinding: Some(Fr::from_str(&transfer.receiver_blinding.to_string()).unwrap()),
        transfer_blinding: Some(Fr::from_str(&transfer.transfer_blinding.to_string()).unwrap()),
    };

    let rng = &mut thread_rng();
    let params = generate_random_parameters::<Bn256, _, _>(circuit.clone(), rng).expect("Failed to generate parameters");

    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, rng).expect("Failed to create proof");

    let public_input = vec![];  .
    verify_proof(&pvk, &proof, &public_input).is_ok()
}

fn main() {
    let is_valid = generate_private_transfer_proof();
    println!("Private transfer proof valid: {}", is_valid);
}