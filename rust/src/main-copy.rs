use std::env;
use std::fs::File;
use std::io::{BufReader, Read, BufRead, BufWriter, Write};


fn main() {
    let args: Vec<String> = env::args().collect();

    let in_path = &args[1];
    let out_path = &args[2];

    let in_file = File::open(in_path)
        .expect("Unable to create input file");
    //let in_file = BufReader::new(in_file);

    let out_file = File::create(out_path).expect("Unable to create file");
    let mut out_file = BufWriter::new(out_file);

    let mut idx = 0;
    let mut row: String = String::from("");

    for line_raw in in_file.bytes() {
        // Just copy the file
        out_file.write_all(&[line_raw.unwrap()])
            .expect("Unable to write data");
    }
}
