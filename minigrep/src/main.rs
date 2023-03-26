extern crate minigrep;

use std::env;
//use std::fs::File;
//use std::io::prelude::*;
use std::process;
//use std::error::Error;

use minigrep::Config;
use minigrep::run;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }

}
