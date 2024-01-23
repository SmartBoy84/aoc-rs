#![allow(dead_code)]
use std::collections::HashMap;

struct Folder {
    size: usize,
    nest: HashMap<String, Folder>, // [Name: Folder]
}

enum ExitStatus<T> {
    EoF(T),
    TraverseOut(T),
}

impl Folder {
    fn create(input: &str) -> ExitStatus<Self> {
        let folder = Folder {
            size: 0,
            nest: HashMap::new(),
        };
        match folder.populate(input) {
            ExitStatus::EoF(_) => ExitStatus::EoF(folder),
            ExitStatus::TraverseOut(_) => ExitStatus::TraverseOut(folder),
        }
    }
    fn populate(&self, input: &str) -> ExitStatus<&Self> {
        ExitStatus::EoF(self)
    }
}

pub fn main(input: &str) -> (usize, usize) {
    let system = Folder::create(input);
    todo!("Ugh, I don't like this day")
}
