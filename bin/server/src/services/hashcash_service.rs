use chrono::{Duration, Utc};
use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::seq::SliceRandom;

use common_pow::HashCash;

use crate::errors::ServerResult;
use crate::models::ExtStruct;

const SECRET_KEY: &str = "Q9tV!MtHaNUCAUg4";
pub struct HardcodedHashcashService;

impl HardcodedHashcashService {
    pub fn validate<'a>(hashcash: &'a HashCash<'a>) -> ServerResult<bool> {
        let mc = new_magic_crypt!(SECRET_KEY.clone(), 256);
        let ext_json = mc.decrypt_base64_to_string(&hashcash.ext)?;
        let ext: ExtStruct = serde_json::from_str(&ext_json)?;

        Ok(ext.bits == hashcash.bits
            && ext.timestamp.timestamp() == (hashcash.timestamp as i64)
            && ext.timestamp > Utc::now())
    }

    pub fn generate_pazzle() -> ServerResult<String> {
        let mc = new_magic_crypt!(SECRET_KEY.clone(), 256);
        let expires_at = Utc::now() + Duration::minutes(2);

        let bits = 5;
        let ext: String = {
            let ext = ExtStruct {
                timestamp: expires_at,
                bits: bits,
            };
            let ext_json = serde_json::to_string(&ext)?;

            mc.encrypt_str_to_base64(ext_json)
        };

        // v = 1, counter = 0
        let pazzle = format!(
            "1:{}:{}:{}:{}::MA==",
            bits,
            expires_at.timestamp(),
            "quotes",
            ext
        );
        Ok(pazzle)
    }
}

pub mod test {
    use common_pow::HashCash;

    use super::HardcodedHashcashService;

    #[test]
    fn test() {
        let puzzle = HardcodedHashcashService::generate_pazzle().unwrap();
        let res = HashCash::try_from(puzzle.as_str());
        assert!(res.is_ok());

        let hashcash = res.unwrap();
        let is_valid_res = HardcodedHashcashService::validate(&hashcash);
        assert!(is_valid_res.is_ok());
        assert!(is_valid_res.unwrap());
    }
}
