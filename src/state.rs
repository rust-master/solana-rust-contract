use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Adder {
    pub a: u64,
    pub b: u64,
    pub result: u64,
}
