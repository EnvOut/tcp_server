use std::str::FromStr;

use anyhow::Context;
use regex::{Captures, Regex};

use crate::errors::CrateResult;
use crate::HashCash;

lazy_static::lazy_static! {
    pub static ref HASHCASH_RE: Regex = Regex::new(r"(?P<ver>\d+)?:(?P<bits>\d+)?:(?P<timestamp>\d+)?:(?P<resource>\S+)?:(?P<ext>\S+)?:(?P<rand>\S+)?:(?P<counter>\S+)?").unwrap();
}

impl<'a> TryFrom<&'a str> for HashCash<'a> {
    type Error = crate::errors::CommonPowErrors;

    fn try_from(original_row: &'a str) -> Result<Self, Self::Error> {
        let captures = verify_and_parse_pazzle(&original_row)?;

        let ver = captures.name("ver").as_u32()?.unwrap_or(1);
        let bits = captures.name("bits").as_u32()?.unwrap_or(1);
        let timestamp = captures.name("timestamp").as_u32()?.unwrap_or(1);
        let resource = captures.name("resource").as_string().unwrap_or_default();
        let ext = captures.name("ext").as_string().unwrap_or_default();
        let rand = captures.name("rand").as_string().unwrap_or_default();
        let counter = if let Some(value) = captures
            .name("counter")
            .map(|it| it.as_str())
            .filter(|it| !(it.trim()).is_empty())
        {
            let decoded_bytes = base64::decode(value)?;
            let counter_str = String::from_utf8(decoded_bytes)?;
            u128::from_str(&counter_str)?
        } else {
            0
        };

        let hash_cash: HashCash<'a> = HashCash {
            ver,
            bits,
            timestamp,
            resource,
            ext,
            rand,
            counter,
            original_row,
        };
        Ok(hash_cash)
    }
}

pub fn verify_and_parse_pazzle(pazzle: &str) -> CrateResult<Captures> {
    HASHCASH_RE.captures(pazzle).ok_or_else(|| {
        crate::errors::CommonPowErrors::WrongHashCash("can't validate pazzle".to_string())
    })
}

pub fn verify_pazzle_and_parse_bits(pazzle_row: &str) -> CrateResult<u32> {
    let captures = verify_and_parse_pazzle(pazzle_row)?;
    let bits = captures
        .name("bits")
        .context("expect named group: \"bits\"")?
        .as_str();
    u32::from_str(bits).map_err(|_| {
        crate::errors::CommonPowErrors::WrongHashCash(format!("Can't parse bits: {:?}", bits))
    })
}

trait OptionAsType<'a, T: Sized> {
    type Err;

    fn as_str(&self) -> Option<&'a str>;
    fn as_string(&self) -> Option<String>;
    fn as_u32(&self) -> Result<Option<u32>, Self::Err>;
}

impl<'a> OptionAsType<'a, regex::Match<'_>> for Option<regex::Match<'a>> {
    type Err = crate::errors::CommonPowErrors;

    fn as_str(&self) -> Option<&'a str> {
        self.map(|it| it.as_str())
    }

    fn as_string(&self) -> Option<String> {
        self.map(|it| it.as_str().to_owned())
    }

    fn as_u32(&self) -> Result<Option<u32>, Self::Err> {
        if let Some(value) = self {
            let value = u32::from_str(value.as_str())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

mod tests {
    use crate::HashCash;

    #[test]
    fn parse_sample1_success() {
        let hashcash_str = "1:5:380119:calvin@comics.net:::MTIz";
        let hashcash = HashCash::try_from(hashcash_str);
        assert!(hashcash.is_ok());

        let hashcash = hashcash.unwrap();

        assert_eq!(hashcash.try_into_string().unwrap_or_default(), hashcash_str);
        println!("hashcash: {:?}", hashcash);
    }

    #[test]
    fn parse_sample2_success() {
        let hashcash =
            HashCash::try_from("1:20:1303030600:anni@cypherspace.org::McMybZIhxKXu57jd:MTIz");
        println!("hashcash: {:?}", hashcash);
    }
}
