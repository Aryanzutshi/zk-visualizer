use crate::hash_functions::HashFunction;
use num_bigint::BigInt;

pub struct MerkleTree<H: HashFunction> {
    pub root: BigInt,
    pub leaves: Vec<BigInt>,
    hash_function: H,
}

impl<H: HashFunction> MerkleTree<H> {
    pub fn new(leaves: Vec<BigInt>, hash_function: H) -> Self {
        assert!(!leaves.is_empty(), "Merkle tree cannot be empty");

        let root = MerkleTree::compute_root(&leaves, &hash_function);
        MerkleTree {
            root,
            leaves,
            hash_function,
        }
    }

    /// Returns a Merkle authentication path for `index`.
    /// Each element: (sibling_hash, sibling_is_left)
    pub fn merkle_path(&self, index: usize) -> Vec<(BigInt, bool)> {
        assert!(index < self.leaves.len(), "Leaf index out of bounds");

        let mut path = Vec::new();
        let mut idx = index;

        let mut level = self.leaves.clone();

        while level.len() > 1 {
            // pad odd count
            if level.len() % 2 != 0 {
                level.push(level.last().unwrap().clone());
            }

            let mut next = Vec::with_capacity((level.len() + 1) / 2);

            for pair in level.chunks(2) {
                next.push(self.hash_function.hash(&pair[0], &pair[1]));
            }

            // sibling
            let is_left = idx % 2 == 1;
            let sibling_idx = if is_left { idx - 1 } else { idx + 1 };

            path.push((level[sibling_idx].clone(), is_left));

            idx /= 2;
            level = next;
        }

        path
    }

    fn compute_root(leaves: &[BigInt], hash_function: &H) -> BigInt {
        let mut level = leaves.to_vec();

        while level.len() > 1 {
            // pad odd count
            if level.len() % 2 != 0 {
                level.push(level.last().unwrap().clone());
            }

            level = level
                .chunks(2)
                .map(|pair| hash_function.hash(&pair[0], &pair[1]))
                .collect();
        }

        level[0].clone()
    }
}
