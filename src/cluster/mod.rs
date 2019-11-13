use crate::utils::lines;
use crate::utils::Combinations;
use clap::ArgMatches;
use rayon::prelude::*;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;

mod jaro;
mod levenshtein;

pub use jaro::Jaro;
pub use levenshtein::Levenshtein;

pub struct Cluster<'a> {
    clusters: Vec<Vec<&'a String>>,
}

impl<'a> Cluster<'a> {
    pub fn pairwise(pairs: Vec<(&'a String, &'a String)>, total: &'a Vec<String>) -> Cluster<'a> {
        let mut clusters: Vec<Vec<&'a String>> = Vec::new();
        'pairwise: for (first, second) in pairs {
            for cluster in &mut clusters {
                if cluster.contains(&first) {
                    cluster.push(second);
                    continue 'pairwise;
                } else if cluster.contains(&second) {
                    cluster.push(first);
                    continue 'pairwise;
                }
            }

            clusters.push(vec![first, second]);
        }

        let mut singletons: Vec<Vec<&'a String>> = Vec::new();

        'singleton: for element in total {
            for cluster in &clusters {
                if cluster.contains(&element) {
                    continue 'singleton;
                }
            }

            singletons.push(vec![element]);
        }

        clusters.extend(singletons);

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

        Cluster::pairwise(pairs, lines)
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
    for cluster in &results.clusters {
        for line in cluster {
            let _ = writeln!(&mut out, "{}", line);
        }
        let _ = writeln!(&mut out, "");
    }
}
