use num_bigint::BigInt;

pub trait HashFunction {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt;
}

pub struct SimpleHash;

impl HashFunction for SimpleAddHash {
    fn hash(&self, a: &BigInt, b: &Bigint) -> BigInt {
        a + b
    }
}

pub struct Customhash;

impl HashFunction for Customhash {
    fn hash(&self, a: &BigInt, b: &BigInt) -> BigInt {
        a + b
    }
}
