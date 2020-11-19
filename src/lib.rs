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
            // \s requires Unicode support
            static ref PARSE_REGEX: Regex = Regex::new(r#"(!([A-Za-z0-9]+([ \t\r\n\f\u00A0]|$)))"#).unwrap();
        }
        let range = PARSE_REGEX.find(s)?.range();
        if !s.is_char_boundary(range.start) || !s.is_char_boundary(range.start + 1) || !s.is_char_boundary(range.end) {
            return None;
        }
        let mut search = String::from(s);
        search.replace_range(range.clone(), "");
        Some((
            bangs::lookup_bang(s[(range.start + 1)..range.end].trim())?,
            search,
        ))
    }
    pub fn with_query(&self, q: &str) -> String {
        self.uri.replace("{{{s}}}", &urlencoding::encode(q))
    }
}
