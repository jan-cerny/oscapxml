#![allow(dead_code)]

extern crate clap;
extern crate minidom;

use clap::Parser;
use minidom::Element;
use minidom::quick_xml;
use std::process;

mod sds;
mod utils;
mod xccdf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to the SCAP source data stream
    filepath: String,
}

fn main() {
    let args = Args::parse();
    let mut reader = quick_xml::Reader::from_file(&args.filepath).expect("Failed to open the input file");
    let root = Element::from_reader(&mut reader).unwrap();
    let result = sds::DataStreamCollection::from_xml(&root);
    match result {
        Ok(data_stream_collection) => {
            data_stream_collection.print_information();
        }
        Err(error) => {
            println!(
                "Failed to parse SCAP Source data stream file '{}': {}",
                args.filepath, error
            );
            process::exit(1);
        }
    }
}
