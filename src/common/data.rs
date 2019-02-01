
const sudachi_splitter1: &str = "\t";
const sudachi_splitter2: &str = ",";

trait MorphAnalyzed {
    fn is_dependent(&self) -> bool;
}

pub struct Morphome {
    pub surface: String,
    pub pos1: String,
    pub pos: String,
    pub normform: String
}

impl Morphome{  
    pub fn new(surface: String, pos1: String, pos: String, normform: String) -> Self {
        Morphome{surface: surface, pos1: pos1, pos: pos, normform: normform}
    }

    pub fn new_from_sudachi(line: String) -> Self {
        let elm = line.as_str().split(sudachi_splitter1).map(|col| col.to_string()).collect::<Vec<String>>();
        let pos1 = elm[1].split(sudachi_splitter2).collect::<Vec<&str>>()[0];
        Self::new(elm[0].clone(), pos1.to_string(), elm[1].clone(), elm[2].clone())
    }

    pub fn new_from_mecab(line: String) -> Self {
        panic!("not implemented")
    }
}