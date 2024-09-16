use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{Circuit, ConstraintSystem, Error},
    poly::Rotation,
    pasta::{Fp, EqAffine},
    dev::MockProver,
    plonk::{create_proof, verify_proof, keygen_pk, keygen_vk, ProvingKey, VerifyingKey},
};
use rand::rngs::OsRng;
use halo2curves::pasta::vesta;
use group::ff::Field;

#[derive(Default)]
struct SimpleCircuit {
    a: Value<Fp>,
    b: Value<Fp>,
    c: Value<Fp>,
}

