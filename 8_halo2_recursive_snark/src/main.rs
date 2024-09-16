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

impl Circuit<Fp> for SimpleCircuit {
    type Config = ();

    fn configure(meta: &mut ConstraintSystem<Fp>) -> Self::Config {
        let a = meta.advice_column();
        let b = meta.advice_column();
        let c = meta.advice_column();
        let sum = meta.advice_column();
        let product = meta.advice_column();

        meta.create_gate("a + b = sum", |meta| {
            let a = meta.query_advice(a, Rotation::cur());
            let b = meta.query_advice(b, Rotation::cur());
            let sum = meta.query_advice(sum, Rotation::cur());

            vec![a + b - sum]
        });

        meta.create_gate("sum * c = product", |meta| {
            let sum = meta.query_advice(sum, Rotation::cur());
            let c = meta.query_advice(c, Rotation::cur());
            let product = meta.query_advice(product, Rotation::cur());

            vec![sum * c - product]
        });

        Self::Config {}
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fp>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "simple circuit",
            |mut region| {
                let a = self.a.unwrap();
                let b = self.b.unwrap();
                let c = self.c.unwrap();

                let sum = a + b;
                let product = sum * c;

                region.assign_advice(|| "a", 0, 0, || Ok(a))?;
                region.assign_advice(|| "b", 1, 0, || Ok(b))?;
                region.assign_advice(|| "c", 2, 0, || Ok(c))?;
                region.assign_advice(|| "sum", 3, 0, || Ok(sum))?;
                region.assign_advice(|| "product", 4, 0, || Ok(product))?;

                Ok(())
            },
        )
    }
}

fn main() {
    let circuit = SimpleCircuit {
        a: Value::known(Fp::from(2)),
        b: Value::known(Fp::from(3)),
        c: Value::known(Fp::from(4)),
    };

    let prover = MockProver::run(5, &circuit, vec![]).unwrap();

    assert_eq!(prover.verify(), Ok(()));
    println!("Initial circuit verification passed.");

}
