use crate::utils::lines;
use crate::utils::Combinations;
use clap::ArgMatches;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;

mod jaro;
mod levenshtein;

pub use jaro::Jaro;
pub use levenshtein::Levenshtein;

pub struct Cluster<'a> {
    clusters: Vec<HashSet<&'a String>>,
}

impl<'a> Cluster<'a> {
    pub fn pairwise(pairs: Vec<(&'a String, &'a String)>) -> Cluster<'a> {
        let mut clusters: Vec<HashSet<&'a String>> = Vec::new();
        'pairwise: for (first, second) in pairs {
            for cluster in &mut clusters {
                match (cluster.contains(&first), cluster.contains(&second)) {
                    (true, false) => {
                        cluster.insert(second);
                        continue 'pairwise;
                    }
                    (false, true) => {
                        cluster.insert(first);
                        continue 'pairwise;
                    }
                    (true, true) => {
                        continue 'pairwise;
                    }
                    (false, false) => (),
                }
            }

            let mut new_set = HashSet::new();
            new_set.insert(first);
            new_set.insert(second);
            clusters.push(new_set);
        }

        Cluster { clusters }
    }
}

pub trait ClusterAlgo: Sized + Sync {
    fn from_matches(matches: &ArgMatches) -> Self;
    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool;

    fn run(matches: &ArgMatches) {
        run_cluster(&matches, Self::from_matches(&matches))
    }

    fn cluster<'a>(&self, lines: &'a Vec<String>) -> Cluster<'a> {
        let pairs = Combinations::new(lines)
            .par_bridge()
            .filter(|(a, b)| self.accept(a, b))
            .collect::<Vec<(&String, &String)>>();

        Cluster::pairwise(pairs)
    }
}

pub fn run_cluster<CA>(matches: &ArgMatches, algo: CA)
where
    CA: ClusterAlgo,
{
    let lines = lines(matches.value_of("file"));
    let clusters = algo.cluster(&lines);

    match matches.value_of("output") {
        None => print_cluster(stdout(), &clusters),
        Some(output) => {
            let file = match File::create(output) {
                Ok(f) => f,
                Err(err) => {
                    eprintln!("Failed to open '{}': {}", output, err);
                    exit(1)
                }
            };

            print_cluster(file, &clusters);
        }
    }
}

pub fn print_cluster<W: Write>(mut out: W, results: &Cluster) {
    let mut first = false;
    for cluster in &results.clusters {
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
