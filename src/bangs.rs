use crate::Bang;

macro_rules! bangs {
    { $($iname:ident : { name: $name:expr, bangs: [$($bang:expr),+], uri: $uri:expr, }),+ } => {
        $(
            const $iname: Bang = Bang {
                name: $name,
                uri: $uri,
            };
        )+
        pub fn lookup_bang(s: &str) -> Option<Bang> {
            match s {
                $(
                    $(
                        $bang => Some($iname),
                    )+
                )+
                _ => None,
            }
        } 
    }
}

include!("db.rs");
