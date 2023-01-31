use std::str::FromStr;

use anyhow::Context;
use rayon::prelude::*;

use common_pow::keccak256_hash;
use common_pow::pazzle_utils::replace_counter;

use crate::errors::ClientResult;

pub fn find_proof(pazzle_row: String, bits: u32) -> ClientResult<Option<(String, String, String)>> {
    println!("pazzle_row: {}", pazzle_row);
    let correct_answer_prefix = "0".repeat(bits as usize);

    let answer: Option<(String, String, String)> = (0..=usize::MAX)
        .into_par_iter()
        .map(|it| it.to_string())
        .map(|counter| (base64::encode(&counter), counter))
        .map(|(encoded_counter, counter)| (replace_counter(&pazzle_row, &encoded_counter), counter))
        .map(|(it, counter)| {
            let hash = keccak256_hash(&it);
            (it, counter, hash)
        })
        .find_any(|(supposed_answer, counter, hash)| hash.starts_with(&correct_answer_prefix));
    println!("found answer: {:?}", answer);
    Ok(answer)
}

mod test {
    use crate::prover::find_proof;

    #[test]
    fn rr() {
        let x = find_proof(
            "1:5:1303030600:anni@cypherspace.org::McMybZIhxKXu57jd:MTIz".to_string(),
            5,
        )
        .unwrap();
        println!()
    }
}
