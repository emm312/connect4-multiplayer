pub mod board;
pub mod server;
pub mod client;

use std::io::{stdout, stdin, Write};

pub fn input() -> String {
    stdout().flush().unwrap();
    let mut out = String::new();
    stdin().read_line(&mut out).unwrap();
    String::from(out.trim())
}