extern crate Mgrep;

use std::env;
//use std::fs::File;
//use std::io::prelude::*;
use std::process;
//use std::error::Error;

use Mgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //let query = &args[1];
    //let filename = &args[2];

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = Mgrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}