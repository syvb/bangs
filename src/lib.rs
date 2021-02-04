use regex::Regex;
use lazy_static::lazy_static;

mod bangs;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Bang {
    pub name: &'static str,
    pub uri: &'static str,
}

impl Bang {
    pub fn parse_search(s: &str) -> Option<(Self, String)> {
        lazy_static! {
            // \s requires Unicode support
            static ref PARSE_REGEX: Regex = Regex::new(r#"(!([a-z0-9]+([ \t\r\n\f\u00A0]|$)))"#).unwrap();
        }
        let s = s.to_ascii_lowercase();
        let range = PARSE_REGEX.find(&s)?.range();
        if !s.is_char_boundary(range.start) || !s.is_char_boundary(range.start + 1) || !s.is_char_boundary(range.end) {
            return None;
        }
        let mut search_without_bang = s.clone();
        search_without_bang.replace_range(range.clone(), "");
        Some((
            bangs::lookup_bang(s[(range.start + 1)..range.end].trim())?,
            search_without_bang,
        ))
    }
    pub fn with_query(&self, q: &str) -> String {
        self.uri.replace("{{{s}}}", &urlencoding::encode(q))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! nomatch_tests {
        ($($name:ident => [$($search:literal),+]),+,) => {
            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!(Bang::parse_search($search), None);
                    )+
                }
            )+
        };
    }

    nomatch_tests! [
        no_expoint => ["hlduwp", "yltdw dbuwy", "dnyw.dwfd"],
        ex_end => ["hi!", "a b!", "tdw hni ! ", "g!", "g! "],
        space_gap => ["dfwp ! g", "dly ! hlduyfp", "! g", "g !", "b! g", "b ! g"],
        whitespace_gap => ["!\tg", "!\rg"],
    ];
}
