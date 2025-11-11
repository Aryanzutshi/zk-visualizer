use crate::hash_functions::HashFunction;
use crate::r1cs::{Operation, R1CS, Variable};
use num_bigint::BigInt;
use std::fs::File;
use std::io::{Read, Write};

pub enum Gate {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Hash(usize, usize, usize),
}

pub struct Circuit {
    hash_function: Option<Box<dyn HashFunction>>,
    inputs: Vec<BigInt>,
    gates: Vec<Gate>,
    outputs: Vec<BigInt>,
}

impl Circuit {
    pub fn new(hash_function: Option<Box<dyn HashFunction>>) -> Self {
        Circuit {
            hash_function,
            inputs: Vec::new(),
            gates: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, value: BigInt) -> usize {
        let index = self.inputs.len();
        self.inputs.push(value);
        index
    }

    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn add_output(&mut self, value: BigInt) {
        self.outputs.push(value);
    }

    fn apply_hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        match &self.hash_function {
            Some(h) => h.hash(a, b),
            None => a + b,  // fallback
        }
    }

    pub fn get_input(&self, index: usize) -> Option<&BigInt> {
        self.inputs.get(index)
    }

    pub fn generate_proof(&self, proof_file: &str) -> std::io::Result<()> {
        let mut r1cs = R1CS::new();

        // Load inputs into R1CS variable list
        r1cs.variables = self
            .inputs
            .iter()
            .enumerate()
            .map(|(i, v)| Variable {
                index: i,
                value: v.clone(),
            })
            .collect::<Vec<_>>();

        for gate in &self.gates {
            match gate {
                Gate::Add(a, b, out) => {
                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(Variable::new(*out, BigInt::default()), BigInt::from(1))],
                        Operation::Add,
                    );
                }

                Gate::Mul(a, b, out) => {
                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(Variable::new(*out, BigInt::default()), BigInt::from(1))],
                        Operation::Mul,
                    );
                }

                Gate::Hash(a, b, out) => {
                    let h = self.apply_hash(&self.inputs[*a], &self.inputs[*b]);

                    // Ensure output variable exists
                    if *out >= r1cs.variables.len() {
                        r1cs.variables.push(Variable::new(*out, h.clone()));
                    } else {
                        r1cs.variables[*out].value = h.clone();
                    }

                    r1cs.add_constraint(
                        vec![(r1cs.variables[*a].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*b].clone(), BigInt::from(1))],
                        vec![(r1cs.variables[*out].clone(), BigInt::from(1))],
                        Operation::Hash,
                    );

                    println!(
                        "Hash gate: a={}, b={}, hash={}, out={}",
                        self.inputs[*a], self.inputs[*b], h, out
                    );
                }
            }
        }

        let satisfied = r1cs.is_satisfied(|a, b| {
            if let Some(h) = &self.hash_function {
                h.hash(a, b)
            } else {
                a + b
            }
        });

        let mut file = File::create(proof_file)?;
        file.write_all(&[satisfied as u8])?;

        println!("Proof generated → {}", proof_file);
        Ok(())
    }

    pub fn verify_proof(&self, proof_file: &str) -> std::io::Result<bool> {
        let mut file = File::open(proof_file)?;
        let mut buf = [0u8];
        file.read_exact(&mut buf)?;
        let valid = buf[0] == 1;
        println!("Proof verification: {}", valid);
        Ok(valid)
    }
}
