use std::str::FromStr;

use hex::ToHex;

use errors::CommonPowErrors;

use crate::errors::CrateResult;

pub mod errors;
pub mod parser;
pub mod pazzle_utils;
pub mod verifier;

pub fn keccak256_hash(data: &str) -> String {
    use tiny_keccak::{Hasher, Keccak};
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data.as_bytes());
    hasher.finalize(&mut output);
    output.encode_hex()
}

/// based on the https://en.wikipedia.org/wiki/Hashcash#Technical_details
#[derive(Debug, Clone)]
pub struct HashCash<'a> {
    //// ver: Hashcash format version, 1 (which supersedes version 0).
    pub ver: u32,
    /// zeros count
    /// bits: Number of "partial pre-image" (zero) bits in the hashed code.
    pub bits: u32,
    /// date: The time that the message was sent, in the format YYMMDD[hhmm[ss]].
    pub timestamp: u32,
    /// resource: Resource data string being transmitted, e.g., an IP address or email address.
    pub resource: String,
    /// ext: Extension (optional; ignored in version 1).
    pub ext: String,
    /// rand: String of random characters, encoded in base-64 format.
    pub rand: String,
    /// counter: Binary counter, encoded in base-64 format.
    pub counter: u128,

    // inner: Option<LazyCell<HashCash>>,
    pub original_row: &'a str,
}

impl<'a> HashCash<'a> {
    pub fn new(
        ver: u32,
        bits: u32,
        timestamp: u32,
        resource: String,
        ext: String,
        rand: String,
        counter: u128,
        original_row: &'a str,
    ) -> Self {
        Self {
            ver,
            bits,
            timestamp,
            resource,
            ext,
            rand,
            counter,
            original_row,
        }
    }

    pub fn try_into_string(&self) -> CrateResult<String> {
        let result: String = self.try_into()?;
        Ok(result)
    }

    pub fn with_counter(&self, counter: u128) -> HashCash<'a> {
        let mut this = self.clone();
        this.counter = counter;
        this
    }
}

impl<'a> TryInto<String> for &'a HashCash<'a> {
    type Error = CommonPowErrors;

    fn try_into(self) -> CrateResult<String> {
        let counter_str = self.counter.to_string();
        let counter = base64::encode(counter_str.into_bytes());
        let row = format!(
            "{ver}:{bits}:{timestamp}:{resource}:{ext}:{rand}:{counter}",
            ver = self.ver,
            bits = self.bits,
            timestamp = self.timestamp,
            resource = self.resource,
            ext = self.ext,
            rand = self.rand,
            counter = counter
        );
        Ok(row)
    }
}
