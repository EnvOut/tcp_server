use regex::Regex;

lazy_static::lazy_static! {
    pub static ref COUNTER_RE: Regex = Regex::new(r"[^:]+$").unwrap();
}

pub fn replace_counter(pazzle_row: &str, counter: &str) -> String {
    COUNTER_RE.replace(pazzle_row, counter).to_string()
}
