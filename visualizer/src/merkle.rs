use crate::hash_functions::HashFunction;
use num_bigint::BigInt;

pub struct MerkleTree<H: HashFunction> {
    pub root: BigInt,
    pub leaves: Vec<BigInt>,
    hash_function: H,
}

impl<H: HashFunction> MerkleTree<H> {
    pub fn new(leaves: Vec<BigInt>, hash_function: H) -> Self {
        let root = MerkleTree::compute_root(&leaves, &hash_function);
        MerkleTree {
            root,
            leaves,
            hash_function,
        }
    }

    pub fn merkle_path(&self, index: usize) -> Vec<(BigInt, bool)> {
        let mut path = Vec::new();
        let mut current_index = index;
        let mut nodes = self.leaves.clone();

        while nodes.len() > 1 {
            let next_level: Vec<BigInt> = nodes
                .chunks(2)
                .map(|chunk| {
                    if chunk.len() == 2 {
                        self.hash_function.hash(&chunk[0], &chunk[1])
                    } else {
                        chunk[0].clone()
                    }
                })
                .collect();

            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            if sibling_index < nodes.len() {
                path.push((nodes[sibling_index].clone(), current_index % 2 == 0));
            }

            current_index /= 2;
            nodes = next_level;
        }

        path
    }

    fn compute_root(leaves: &Vec<BigInt>, hash_function: &H) -> BigInt {
        let mut nodes = leaves.clone();
        while nodes.len() > 1 {
            nodes = nodes
                .chunks(2)
                .map(|chunk| {
                    if chunk.len() == 2 {
                        hash_function.hash(&chunk[0], &chunk[1])
                    } else {
                        chunk[0].clone()
                    }
                })
                .collect();
        }
        nodes[0].clone()
    }
}
