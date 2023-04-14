use std::{fs, str};

use serde::Deserialize;

// IO
pub fn hello() {
    println!("Hello");
}

#[derive(Deserialize, Debug)]
pub struct Paper {
    pub identifier: i64,
    pub paper_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Entity {
    pub index: u64,
    pub token: String,
    pub var_type: String,
    pub domain: String,
    pub context: String,
}

#[derive(Deserialize, Debug)]
pub struct DAGEdge {
    pub start: i64,
    pub end: i64,
}

#[derive(Deserialize, Debug)]
pub struct DAGGraph {
    pub entities: Vec<Entity>,
    pub edges: Vec<DAGEdge>,
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub paper: Paper,
    pub graph: DAGGraph,
}

pub fn parse_annotation(filepath: &str) -> Annotation {
    let buf = fs::read(filepath).expect("Unable to read annotation file");
    let s = match str::from_utf8(&buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let annot = toml::from_str(s).unwrap();
    annot
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use crate::parse_annotation;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_parse_annotation() {
        let filename = "test_data/annot.toml";
        let annot = parse_annotation(filename);
        assert_eq!(annot.paper.identifier, 15706789);
    }
}
