use regex::Regex;
use lazy_static::lazy_static;

mod bangs;

#[derive(Debug, Copy, Clone)]
pub struct Bang {
    pub name: &'static str,
    uri: &'static str,
}

impl Bang {
    pub fn parse_search(s: &str) -> Option<(Self, String)> {
        lazy_static! {
            static ref PARSE_REGEX: Regex = Regex::new(r#"(!([A-Za-z0-9]+($|\s)))"#).unwrap();
        }
        let captures = PARSE_REGEX.captures(s)?;
        let match1 = captures.get(1)?;
        let match2 = captures.get(2)?;
        Some((
            bangs::lookup_bang(match2.as_str().trim())?,
            {
                let range = match1.range();
                
                if !s.is_char_boundary(range.start) || !s.is_char_boundary(range.end) {
                    return None;
                }
                let prebang = s.split_at(range.start).0;
                let postbang = s.split_at(range.end).1;
                
                let mut result = String::with_capacity(prebang.len());
                result.push_str(prebang);
                result.push_str(postbang);
                result
            }
        ))
    }
    pub fn with_query(&self, q: &str) -> String {
        self.uri.replace("{{{s}}}", &urlencoding::encode(q))
    }
}
