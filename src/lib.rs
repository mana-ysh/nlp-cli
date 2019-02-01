extern crate clap;

use clap::ArgMatches;
use std::borrow::Cow;
use std::error::Error;
use std::fs::File;
use std::io::{Write, Read, BufWriter, BufReader, Lines, copy, stdin, stdout, StdoutLock, StdinLock};
use std::io::prelude::*;
use std::process::exit;

const sudachi_splitter1: &str = "\t";
const sudachi_splitter2: &str = ",";
const eos_marker: &str = "EOS";

trait MorphAnalyzed {
    fn is_dependent(&self) -> bool;
}

struct Morphome {
    surface: String,
    pos1: String,
    pos: String,
    normform: String
}

impl Morphome{  
    fn new(surface: String, pos1: String, pos: String, normform: String) -> Self {
        Morphome{surface: surface, pos1: pos1, pos: pos, normform: normform}
    }

    fn new_from_sudachi(line: String) -> Self {
        let elm = line.as_str().split(sudachi_splitter1).map(|col| col.to_string()).collect::<Vec<String>>();
        let pos1 = elm[1].split(sudachi_splitter2).collect::<Vec<&str>>()[0];
        Self::new(elm[0].clone(), pos1.to_string(), elm[1].clone(), elm[2].clone())
    }

    fn new_from_mecab(line: String) -> Self {
        panic!("not implemented")
    }
}

pub fn run(matches: ArgMatches) -> Result<(), Box<Error>> {
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
                    run_wakati(&mut in_buf, &mut out_buf, matches.clone());  // FIXME: clone
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

fn run_wakati<T: BufRead, U: Write>(in_buf: &mut T, out_buf: &mut U, matches: ArgMatches) {

    let mut sent: Vec<Morphome> = Vec::new();
    for line in in_buf.lines() {
        let line = line.unwrap();  // String
        match line.as_str() {
            eos_marker => {
                writeln!(out_buf, "{}", sent.iter().map(|m| m.surface.as_str()).collect::<Vec<&str>>().join(" "));
                sent.clear();
            },
            _ => {
                sent.push(Morphome::new_from_sudachi(line));
            },
        }
    }
}

fn get_bufwriter(filepath: &str) -> BufWriter<File> {
    BufWriter::new(File::create(filepath).unwrap())
}

fn get_bufreader(filepath: &str) -> BufReader<File> {
    BufReader::new(File::open(filepath).unwrap())
}