#[macro_use(crate_version, crate_name)]
extern crate clap;

mod cluster;
mod cluster_output;
mod combinations;
mod doc_reader;
mod utils;
mod validation;

use clap::{App, Arg, SubCommand};
use cluster::ClusterAlgo;
use validation::*;

fn main() {
    let ratio_arg = Arg::with_name("ratio")
        .help("minimum ration to cluster")
        .short("r")
        .long("ratio")
        .required(true)
        .takes_value(true)
        .validator(ratio);

    let winkler_arg = Arg::with_name("winkler")
        .help("use jaro winkler: optimize prefix similarity")
        .short("w")
        .long("winkler");

    let file_arg = Arg::with_name("file")
        .help("read this file for lines")
        .short("f")
        .long("files")
        .takes_value(true);

    let irs_arg = Arg::with_name("irs")
        .help("input record seperator")
        .short("R")
        .long("irs")
        .possible_values(&["line", "l", "paragraph", "p", "null", "n", "0"])
        .takes_value(true);

    let ors_arg = Arg::with_name("ors")
        .help("output record seperator")
        .short("r")
        .long("ors")
        .possible_values(&["0", "line", "l", "double", "d"])
        .takes_value(true);

    let ofs_arg = Arg::with_name("ofs")
        .help("output field seperator")
        .long("ofs")
        .possible_values(&["0", ":", "line", "l"])
        .takes_value(true);

    let threshold_arg = Arg::with_name("threshold")
        .help("maximum edit difference")
        .short("t")
        .long("threshold")
        .required(true)
        .takes_value(true)
        .validator(nonnegative);

    let damerau_arg = Arg::with_name("damerau")
        .help("use damerau levenshtein")
        .short("d")
        .long("damerau");

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .help("clusters incoming lines")
        .subcommand(
            SubCommand::with_name("jaro")
                .alias("j")
                .arg(&ratio_arg)
                .arg(&winkler_arg)
                .arg(&file_arg)
                .arg(&ors_arg)
                .arg(&ofs_arg)
                .arg(&irs_arg),
        )
        .subcommand(
            SubCommand::with_name("levenshtein")
                .alias("l")
                .arg(&threshold_arg)
                .arg(&damerau_arg)
                .arg(&file_arg)
                .arg(&ors_arg)
                .arg(&ofs_arg)
                .arg(&irs_arg),
        )
        .subcommand(
            SubCommand::with_name("normalized-levenshtein")
                .alias("n")
                .arg(&ratio_arg)
                .arg(&damerau_arg)
                .arg(&file_arg)
                .arg(&ors_arg)
                .arg(&ofs_arg)
                .arg(&irs_arg),
        )
        .subcommand(
            SubCommand::with_name("osa")
                .alias("o")
                .arg(&threshold_arg)
                .arg(&file_arg)
                .arg(&ors_arg)
                .arg(&ofs_arg)
                .arg(&irs_arg),
        )
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

        ("osa", Some(matches)) => {
            cluster::OSA::run(&matches);
        }

        _ => println!("{}", matches.usage()),
    }
}
