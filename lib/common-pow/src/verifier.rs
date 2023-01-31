use hex::ToHex;
use tiny_keccak::{Hasher, Keccak};

use crate::errors::CrateResult;
use crate::parser::verify_pazzle_and_parse_bits;

pub fn string_to_256hash(input: &str) -> String {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(input.as_bytes());
    hasher.finalize(&mut output);
    output.encode_hex()
}

pub fn verify_answer_hash(pazzle_answer: &str) -> CrateResult<(bool, String, String)> {
    let bits = verify_pazzle_and_parse_bits(pazzle_answer)?;
    let hash = string_to_256hash(pazzle_answer);
    let prefix = "0".repeat(bits as usize);
    Ok((hash.starts_with(&prefix), prefix, hash))
}
