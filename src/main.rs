extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;

fn main() {
    if args().len() !=3 {
        eprintln!("Usage:'source' 'output'");
        return;
    }
    let mut input = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
    let mut output = File::create(args().nth(2).unwrap()).unwrap();
    let mut encoder = GzEncoder::new(&mut output, Compression::default());
    let start = Instant::now();
    std::io::copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!("Source file size: {} bytes", input.get_ref().metadata().unwrap().len());
    println!("Compressed file size: {} bytes", output.metadata().unwrap().len());
    println!("Compression ratio: {:.2}%", (output.metadata().unwrap().len() as f64 / input.get_ref().metadata().unwrap().len() as f64) * 100.0);
    println!("Elapsed time: {:.2?}", start.elapsed());
}
