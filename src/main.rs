use ilikedags::{hello, parse_annotation};

fn main() {
    hello();

    let filename = "test_data/annot.toml";
    let annot = parse_annotation(filename);
    println!("{:?}", annot);
}
