use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let in_path = &args[1];
    let out_path = &args[2];

    let in_file = File::open(in_path)
        .expect("Unable to create input file");
    let in_file = BufReader::new(in_file);

    let out_file = File::create(out_path).expect("Unable to create file");
    let mut out_file = BufWriter::new(out_file);

    let mut idx = 0;
    let mut row: Vec<String> = vec![];

    for line_raw in in_file.lines().skip(1) {
        let line = line_raw.unwrap();
        if idx < 3 {
            row.push(line[2..].to_string());
            row.push("\t".to_string());
            idx = idx + 1;
        }
        else {
            row.push("\n".to_string());
            let joined = row.join("");
            out_file.write_all(joined.as_bytes())
                .expect("Unable to write data");

            // Clearing variables
            row.clear();
            idx = 0;
        }
    }
}
