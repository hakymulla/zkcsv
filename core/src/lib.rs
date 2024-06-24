use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize, Eq, PartialEq)]
pub struct Output {
    pub first_column_total: i64,
    pub second_column_total: i64,
    pub hash: Digest,
}