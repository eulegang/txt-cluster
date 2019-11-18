use crate::doc_reader::{DocReader, RecordSeperator};
use clap::ArgMatches;
use std::io;

pub fn docs(matches: &ArgMatches) -> Vec<String> {
    match matches.value_of("file") {
        None => {
            let sin = io::stdin();
            DocReader::new(sin.lock(), mode(&matches)).collect()
        }
        Some(path) => match std::fs::File::open(path) {
            Ok(file) => DocReader::with_read(file, mode(&matches)).collect(),
            Err(err) => {
                eprintln!("Error opening '{}': {}", path, err);
                std::process::exit(1);
            }
        },
    }
}

fn mode(matches: &ArgMatches) -> RecordSeperator {
    match matches.value_of("input_mode") {
        Some("paragraph") | Some("p") => RecordSeperator::Paragraph,
        Some("line") | Some("l") | None => RecordSeperator::Line,
        Some("null") | Some("n") | Some("0") => RecordSeperator::Null,
        _ => unreachable!(),
    }
}
