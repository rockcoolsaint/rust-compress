use std::{
    env::args,
    fs::File,
    io::{copy, BufReader},
    time::Instant,
};

use flate2::{write::GzEncoder, Compression};

fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }

    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}, output", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
}
