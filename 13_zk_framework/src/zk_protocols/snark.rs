use bellman::groth16::{generate_random_parameters, create_random_proof, verify_proof, prepare_verifying_key};
use pairing::bn256::{Bn256, Fr};
use bellman::Circuit;
use rand::thread_rng;

pub fn create_snark_proof<C: Circuit<Fr>>(circuit: C) -> bool {
    
    let rng = &mut thread_rng();
    let params = generate_random_parameters::<Bn256, _, _>(circuit.clone(), rng).expect("Failed to generate parameters");

    let pvk = prepare_verifying_key(&params.vk);

    let proof = create_random_proof(circuit, &params, rng).expect("Failed to create proof");

    let public_input = vec![];  
    verify_proof(&pvk, &proof, &public_input).is_ok()
}
