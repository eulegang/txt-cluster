use crate::cluster::{Cluster, ClusterAlgo};
use crate::cluster_output::{ClusterOutput, FieldSeperator as OFS, RecordSeperator as ORS};
use crate::doc_reader::{DocReader, RecordSeperator as IRS};
use clap::ArgMatches;
use std::fs::File;
use std::io;
use std::process::exit;

pub fn docs(matches: &ArgMatches) -> Vec<String> {
    match matches.value_of("file") {
        None => {
            let sin = io::stdin();
            DocReader::new(sin.lock(), irs(&matches)).collect()
        }
        Some(path) => match std::fs::File::open(path) {
            Ok(file) => DocReader::with_read(file, irs(&matches)).collect(),
            Err(err) => {
                eprintln!("Error opening '{}': {}", path, err);
                std::process::exit(1);
            }
        },
    }
}

fn irs(matches: &ArgMatches) -> IRS {
    match matches.value_of("irs") {
        Some("paragraph") | Some("p") => IRS::Paragraph,
        Some("line") | Some("l") | None => IRS::Line,
        Some("null") | Some("n") | Some("0") => IRS::Null,
        _ => unreachable!(),
    }
}

fn ofs(matches: &ArgMatches) -> OFS {
    match matches.value_of("ofs") {
        Some("0") => OFS::Null,
        Some(":") => OFS::Colon,
        Some("line") | Some("l") | None => OFS::Line,
        _ => unreachable!(),
    }
}

fn ors(matches: &ArgMatches) -> ORS {
    match matches.value_of("ors") {
        Some("0") => ORS::Null,
        Some("line") | Some("l") => ORS::Line,
        Some("double") | Some("d") | None => ORS::DLine,
        _ => unreachable!(),
    }
}

pub fn run_cluster<CA>(matches: &ArgMatches, algo: CA)
where
    CA: ClusterAlgo,
{
    let lines = docs(matches);
    let clusters = algo.cluster(&lines);
    print_cluster(matches, clusters);
}

fn print_cluster(matches: &ArgMatches, cluster: Cluster<'_>) {
    match matches.value_of("output") {
        None => {
            ClusterOutput::new(io::stdout(), ofs(matches), ors(matches)).output(cluster);
        }

        Some(path) => {
            let file = match File::create(path) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Failed to open '{}': {}", path, err);
                    exit(1)
                }
            };

            ClusterOutput::new(file, ofs(matches), ors(matches)).output(cluster);
        }
    }
}
