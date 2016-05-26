extern crate clap;
extern crate regex;

use std::io::prelude::*;
use std::io;
use clap::{App, Arg};
use regex::Regex;

fn main() {
    let matches = App::new("bender")
        .version("0.1.0")
        .author("Kilian Koeltzsch <me@kilian.io>")
        .about("kiss my shiny metal ass")
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("Change your bender's name"))
        .get_matches();

    let bender_name = matches.value_of("name").unwrap_or("bender");

    let ping_regex = format!("^{} ping", bender_name);
    let ping_cmd = Regex::new(&ping_regex).unwrap();

    loop {
        print_prompt(bender_name);

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        if ping_cmd.is_match(&input) {
            print_prompt(bender_name);
            println!("PONG");
        }
    }
}

fn print_prompt(name: &str) {
    print!("{}> ", name);
    io::stdout().flush().ok().expect("failed to flush stdout");
}
