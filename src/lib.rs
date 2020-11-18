use regex::Regex;
use lazy_static::lazy_static;

mod bangs;

#[derive(Debug, Clone)]
pub struct Bang {
    pub name: &'static str,
    uri: &'static str,
}

impl Bang {
    pub fn parse_search(s: &str) -> Option<Self> {
        lazy_static! {
            static ref PARSE_REGEX: Regex = Regex::new(r#"!([A-Za-z0-9])+($|\s)"#).unwrap();
        }
        let matc = PARSE_REGEX.captures(s)?.get(1)?;
        bangs::lookup_bang(matc.as_str())
    }
}
