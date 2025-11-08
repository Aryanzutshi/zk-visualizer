use num_bigint::BigInt;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Clone, Serialize, Deserialize)]
pub struct Variable {
    pub index: usize,
    pub value: BigInt,
}

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Add,
    Mul,
    Hash,
}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    pub left: Vec<(Variable, BigInt)>,
    pub right: Vec<(Variable, BigInt)>,
    pub output: Vec<(Variable, BigInt)>,
    pub operation: Operation,
}

#[derive(Serialize, Deserialize)]
pub struct R1CS {
    pub variables: Vec<Variable>,
    pub constraints: Vec<Constraints>,
}

impl R1CS {
    pub fn new() -> Self {
        R1CS {
            variables: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(
        &mut self,
        left: Vec<(Variable, BigInt)>,
        right: Vec<(Variable, BigInt)>,
        output: Vec<(Variable, BigInt)>,
        operation: Operation,
    ) {
        let constraint = Constraint {
            left,
            right,
            output,
            operation,
        };
        self.constraints.push(constraint);
    }

    pub fn is_satisfied<F>(&self, apply_hash: F) -> bool
    where
        F: Fn(&BigInt, &BigInt) -> BigInt,
    {
        for constraint in &self.constraints {
            let left_value: BigInt = constraint
                .left
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let right_value: BigInt = constraint
                .right
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();
            let output_value: BigInt = constraint
                .output
                .iter()
                .map(|(var, coeff)| &var.value * coeff)
                .sum();

            match constraint.operation {
                Operation::Add => {
                    if left_value.clone() + right_value.clone() != output_value {
                        println!(
                            "Addition contraint not satisfied: left + right = {}, but output = {}",
                            left_value + right_value,
                            output_value
                        );
                        return false;
                    }
                }

                Operation::Mul => {
                    if left_value.clone() * right_value.clone() != output_value {
                        println!(
                            "Multiplication contraint not satisfied: left * right = {}, but output = {}",
                            left_value * right_value,
                            output_value
                        );
                        return false;
                    }
                }

                Operation::Hash => {
                    let computed_hash = apply_hash(&left_value, &right_value);
                    if computed_hash != output_value {
                        println!(
                            "hash constraint not satisfied: computed_hash = {},
                                but expected_output = {}", computed_hash, output_value
                        );
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn save_to_binary(&self, filename: &str) {
        let mut file = File::create(filename).expect("Could not create file");
        let data = bincode::serialize(self).expect("Failed to serialize R1CS");
        file.write_all(&data).expect("Failed to write file");
    }
}
