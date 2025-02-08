use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::Fr;
use bellman::gadgets::num::AllocatedNum;

pub struct ArithmeticCircuit {
    pub a: Option<Fr>,
    pub b: Option<Fr>,
    pub c: Option<Fr>,
}

impl Circuit<Fr> for ArithmeticCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a = AllocatedNum::alloc(cs.namespace(|| "a"), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = AllocatedNum::alloc(cs.namespace(|| "b"), || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = AllocatedNum::alloc(cs.namespace(|| "c"), || self.c.ok_or(SynthesisError::AssignmentMissing))?;

       
        let sum = AllocatedNum::alloc(cs.namespace(|| "sum = a + b"), || {
            let mut tmp = self.a.unwrap();
            tmp.add_assign(&self.b.unwrap());
            Ok(tmp)
        })?;
        cs.enforce(|| "a + b = sum", |lc| lc + a.get_variable() + b.get_variable(), |lc| lc + CS::one(), |lc| lc + sum.get_variable());

        let product = AllocatedNum::alloc(cs.namespace(|| "product = sum * c"), || {
            let mut tmp = self.a.unwrap();
            tmp.add_assign(&self.b.unwrap());
            tmp.mul_assign(&self.c.unwrap());
            Ok(tmp)
        })?;
        cs.enforce(|| "sum * c = product", |lc| lc + sum.get_variable(), |lc| lc + c.get_variable(), |lc| lc + product.get_variable());

        Ok(())
    }
}
