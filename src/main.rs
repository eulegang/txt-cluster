#[macro_use(crate_version, crate_name)]
extern crate clap;

mod cluster;
mod utils;
mod validation;

use clap::{App, Arg, SubCommand};
use cluster::ClusterAlgo;
use validation::*;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .help("clusters incoming lines")
        .subcommand(cluster_standard_args(
            SubCommand::with_name("jaro")
                .arg(
                    Arg::with_name("ratio")
                        .help("minimum ration to cluster")
                        .short("r")
                        .long("ratio")
                        .required(true)
                        .takes_value(true)
                        .validator(ratio),
                )
                .arg(
                    Arg::with_name("winkler")
                        .help("use jaro winkler: optimize prefix similarity")
                        .short("w")
                        .long("winkler"),
                ),
        ))
        .subcommand(cluster_standard_args(
            SubCommand::with_name("levenshtein")
                .arg(
                    Arg::with_name("threshold")
                        .help("maximum edit difference")
                        .short("t")
                        .long("threshold")
                        .required(true)
                        .takes_value(true)
                        .validator(nonnegative),
                )
                .arg(
                    Arg::with_name("damerau")
                        .help("use damerau levenshtein")
                        .short("d")
                        .long("damerau"),
                ),
        ))
        .subcommand(cluster_standard_args(
            SubCommand::with_name("normalized-levenshtein")
                .arg(
                    Arg::with_name("ratio")
                        .help("maximum edit difference normalized")
                        .short("r")
                        .long("ratio")
                        .required(true)
                        .takes_value(true)
                        .validator(ratio),
                )
                .arg(
                    Arg::with_name("damerau")
                        .help("use damerau levenshtein")
                        .short("d")
                        .long("damerau"),
                ),
        ))
        .get_matches();

    match matches.subcommand() {
        ("jaro", Some(matches)) => {
            cluster::Jaro::run(&matches);
        }

        ("levenshtein", Some(matches)) => {
            cluster::Levenshtein::run(&matches);
        }

        ("normalized-levenshtein", Some(matches)) => {
            cluster::NormLevenshtein::run(&matches);
        }

        _ => println!("{}", matches.usage()),
    }
}

fn cluster_standard_args<'a, 'b>(cmd: App<'a, 'b>) -> App<'a, 'b> {
    cmd.arg(
        Arg::with_name("file")
            .help("read this file for lines")
            .short("f")
            .long("files")
            .takes_value(true),
    )
}
