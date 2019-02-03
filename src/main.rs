extern crate clap;
extern crate nlpcli;

use clap::{App, Arg, ArgMatches};
use std::error::Error;
use std::fs::File;
use std::io::{Write, BufWriter, BufReader, stdin, stdout};
use std::io::prelude::*;
use std::process::exit;

use nlpcli::task::cooc;
use nlpcli::task::ngram;
use nlpcli::task::wakati;

fn main() {
    let app = App::new("nlp-cli")
                   .version("0.1.0")
                   .arg(Arg::with_name("task")
                        .help("Task [ngram, wakati]")
                        .short("t")
                        .long("task")
                        .takes_value(true))
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
                    .arg(Arg::with_name("n")
                        .help("n-gram's n")
                        .short("n")
                        .long("n")
                        .takes_value(true))
                    .arg(Arg::with_name("window")
                        .help("Window size for calculating co-occurence")
                        .long("window")
                        .short("w")
                        .takes_value(true));
    let matches = app.get_matches();
    if let Err(e) = run(matches) {
        println!("Fail to run: {}", e);
        exit(1);
    }
}

fn run(matches: ArgMatches) -> Result<(), Box<Error>> {
    let sin = stdin();
    let mut sin = sin.lock();
    let mut in_buf: Box<BufRead> = match matches.value_of("input") {
        Some(filepath) => Box::new(get_bufreader(filepath)),
        None => Box::new(BufReader::new(sin))
    };
    
    let out = stdout();
    let mut out = out.lock();
    let mut out_buf: Box<Write> = match matches.value_of("output") {
        Some(filepath) => Box::new(get_bufwriter(filepath)),
        None => Box::new(BufWriter::new(out))
    }; 

    match matches.value_of("task") {
        Some(task) => {
            match task {
               "wakati" => {
                    wakati::run_wakati(&mut in_buf, &mut out_buf);  // FIXME: clone
                }
                "ngram" => {
                    let n = matches.value_of("n").unwrap().parse().unwrap();
                    ngram::output_ngram_stats(&mut in_buf, &mut out_buf, n);

                }
                "cooc" => {
                    let window = matches.value_of("window").unwrap().parse().unwrap();
                    cooc::output_cooc_stats(&mut in_buf, &mut out_buf, window);
                }
                _ => {
                    println!("Invalid subcommand");
                    exit(1);
                }
            }
        },
        None => {
            println!("Please input task argument");
            exit(1);
        } 
    }
    Ok(())
}

fn get_bufwriter(filepath: &str) -> BufWriter<File> {
    BufWriter::new(File::create(filepath).unwrap())
}

fn get_bufreader(filepath: &str) -> BufReader<File> {
    BufReader::new(File::open(filepath).unwrap())
}