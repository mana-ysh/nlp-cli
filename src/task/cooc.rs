use std::{cmp, collections};
use std::io::{BufRead, Write};
use std::iter::FromIterator;

// FIXME: type around windowsize i16? usize?
pub fn output_cooc_stats<T: BufRead, U: Write>(in_buf: &mut T, out_buf: &mut U, window: i16) {
    if (window % 2 == 0) {
        panic!("window size should be odd. not {}.", window);
    }
    let wleft = window / 2;
    let wright = window / 2;
    let mut cooc2freq = collections::HashMap::<String, u32>::new();
    for line in in_buf.lines() {
        let line = line.unwrap();
        let morphs = line.trim().split(" ").collect::<Vec<&str>>();
        let n_morph = morphs.len() as i16;
        for (i, m) in morphs.iter().enumerate() {
            let _i = i as i16;
            let start_idx = cmp::max(0, _i-wleft) as usize;
            let end_idx = cmp::min(n_morph, _i+wright+1) as usize;
            let target = morphs.get(i).unwrap();
            for j in start_idx..end_idx {
                if (i == j) {
                    continue;
                }
                let context = morphs.get(j).unwrap();
                if target < context {
                    (*cooc2freq.entry([target, " ", context].concat()).or_insert(0)) += 1;
                } else {
                    (*cooc2freq.entry([context, " ", target].concat()).or_insert(0)) += 1;
                }
            }
        }
    } 

    let mut cooc2freq_sort = Vec::from_iter(cooc2freq);
    cooc2freq_sort.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for pair in &cooc2freq_sort {
        writeln!(out_buf, "{}\t{}", pair.0, pair.1 / 2);
    }
}