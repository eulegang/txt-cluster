use crate::cluster::{Cluster, ClusterAlgo};
use crate::doc_reader::{DocReader, RecordSeperator};
use clap::ArgMatches;
use std::fs::File;
use std::io::{self, Write};
use std::process::exit;

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

pub fn run_cluster<CA>(matches: &ArgMatches, algo: CA)
where
    CA: ClusterAlgo,
{
    let lines = docs(matches);
    let clusters = algo.cluster(&lines);

    match matches.value_of("output") {
        None => print_cluster(io::stdout(), clusters),
        Some(output) => {
            let file = match File::create(output) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Failed to open '{}': {}", output, err);
                    exit(1)
                }
            };

            print_cluster(file, clusters);
        }
    }
}

pub fn print_cluster<W: Write>(mut out: W, results: Cluster) {
    let mut first = false;
    for cluster in results.into_iter() {
        if !first {
            let _ = writeln!(&mut out, "");
        } else {
            first = true;
        }

        for line in cluster {
            let _ = writeln!(&mut out, "{}", line);
        }
    }
}
