use std::collections;
use std::io::{BufRead, Write};
use std::iter::FromIterator;

pub fn output_ngram_stats<T: BufRead, U: Write>(in_buf: &mut T, out_buf: &mut U, n: usize) {
    let mut ngram2freq = collections::HashMap::<String, u32>::new();
    for line in in_buf.lines() {
        let line = line.unwrap();
        let morphs = line.trim().split(" ").collect::<Vec<&str>>();
        for mseq in morphs.windows(n) {
            (*ngram2freq.entry(mseq.join(" ")).or_insert(0)) += 1;
        }
    } 

    let mut ngram2freq_sort = Vec::from_iter(ngram2freq);
    ngram2freq_sort.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for pair in &ngram2freq_sort {
        writeln!(out_buf, "{}\t{}", pair.0, pair.1);
    }
}