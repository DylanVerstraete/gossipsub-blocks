use serde::{Deserialize, Serialize};
use web3::types::H256;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Attestation is a struct that holds the attestation for a given block
/// ...
pub struct Attestation<H>
where
    H: Serialize,
{
    pub block_hash: H256,
    pub signature: H,
}
