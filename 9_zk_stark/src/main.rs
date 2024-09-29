use winterfell::{
    FieldExtension, ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};
use winterfell::verifier::verify;
use winterfell::Air;
use winterfell::air::AirConfig;

struct FibonacciAir {
    sequence_length: usize,
}

impl AirConfig for FibonacciAir {
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;
}

impl Air for FibonacciAir {
    fn public_inputs(&self) -> Vec<Self::BaseField> {
        vec![BaseElement::new(1)]  
    }

    fn context(&self) -> winterfell::air::Context<Self::BaseField> {
        
        winterfell::air::Context::new(self.sequence_length, ProofOptions::default())
    }
}
struct FibonacciProver {
    options: ProofOptions,
}

impl Prover for FibonacciProver {
    type BaseField = BaseElement;
    type Air = FibonacciAir;
    type Trace = FibonacciTrace;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> Vec<Self::BaseField> {
        trace.get_pub_inputs()
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }

    fn trace(&self) -> Self::Trace {
        FibonacciTrace::new(10)  
    }
}



