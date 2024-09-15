#[derive(Debug, Clone, Copy)]
enum Variable {
    Input(usize),     
    Intermediate(usize),
}

#[derive(Debug, Clone)]
enum Gate {
    Add(Variable, Variable),      
    Mul(Variable, Variable),      
}


struct Circuit {
    inputs: Vec<i64>,     
    gates: Vec<Gate>,     
    intermediates: Vec<i64>,  
}

impl Circuit {
    
    fn new(inputs: Vec<i64>) -> Self {
        Circuit {
            inputs,
            gates: Vec::new(),
            intermediates: Vec::new(),
        }
    }

    
    fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    fn evaluate(&mut self) {
        for gate in &self.gates {
            let result = match gate {
                Gate::Add(var1, var2) => self.get_value(*var1) + self.get_value(*var2),
                Gate::Mul(var1, var2) => self.get_value(*var1) * self.get_value(*var2),
            };
            
            self.intermediates.push(result);
        }
    }

    
    fn get_value(&self, var: Variable) -> i64 {
        match var {
            Gate::Add(var1, var2) => self.get_value(*var1) + self.get_value(*var2),
                Gate::Mul(var1, var2) => self.get_value(*var1) * self.get_value(*var2),
        }
    }

    
    fn output(&self) -> i64 {
        *self.intermediates.last().expect("No gates evaluated")
    }
}

fn main() {
    let inputs = vec![3, 5];
    let mut circuit = Circuit::new(inputs);
    circuit.add_gate(Gate::Add(Variable::Input(0), Variable::Input(1)));
    circuit.add_gate(Gate::Mul(Variable::Intermediate(0), Variable::Input(0)));
    circuit.evaluate();
    println!("Circuit output: {}", circuit.output()); 
}
