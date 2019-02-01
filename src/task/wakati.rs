use std::io::{BufRead, Write};
use common::data::Morphome;
use common::eos_marker;

pub fn run_wakati<T: BufRead, U: Write>(in_buf: &mut T, out_buf: &mut U) {
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