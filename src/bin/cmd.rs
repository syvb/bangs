use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let query = {
        let mut buf = String::new(); 
        stdin.lock().read_to_string(&mut buf).unwrap();
        buf
    };
    dbg!(bangs::Bang::parse_search(&query));
}
