extern crate clap;
extern crate nlpcli;

use clap::{App, Arg, ArgMatches, SubCommand};
use std::process::exit;
use nlpcli::run;


fn main() {
    let app = App::new("nlp-cli")
                   .version("0.1.0")
                   .arg(Arg::with_name("input")
                        .help("Input file")
                        .short("i")
                        .long("input")
                        .takes_value(true))
                    .arg(Arg::with_name("output")
                        .help("Output file")
                        .short("o")
                        .long("output")
                        .takes_value(true))
                    .subcommand(SubCommand::with_name("wakati")
                        .about("generate wakati text file")
                    );
    
    let matches = app.get_matches();
    if let Err(e) = run(matches) {
        println!("Fail to run: {}", e);
        exit(1);
    }
}