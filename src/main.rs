extern crate curl;

use std::io::{stdout, Write};
use std::env;
use curl::easy::Easy;


// Print a web page onto stdout
fn main() {
    let mut url = String::from("https://checkip.amazonaws.com");
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        url.clear();
        url.push_str(&args[1]);
    }

    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    let response_code = easy.response_code().unwrap() as i32;
    if response_code != 200 {
        std::process::exit(response_code);
    }
}
