use crate::combinations::*;
use crate::utils::docs;
use clap::ArgMatches;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::File;
use std::io::{stdout, Write};
use std::process::exit;

mod jaro;
mod levenshtein;
mod norm_levenshtein;
mod osa;

pub use jaro::Jaro;
pub use levenshtein::Levenshtein;
pub use norm_levenshtein::NormLevenshtein;
pub use osa::OSA;

pub struct Cluster<'a> {
    clusters: Vec<HashSet<&'a String>>,
}

impl<'a> Cluster<'a> {
    pub fn pairwise(pairs: Vec<(&'a String, &'a String)>) -> Cluster<'a> {
        let mut clusters: Vec<HashSet<&'a String>> = Vec::new();
        'pairwise: for (first, second) in pairs {
            for cluster in &mut clusters {
                if cluster.contains(&first) || cluster.contains(&second) {
                    cluster.insert(&first);
                    cluster.insert(&second);
                    continue 'pairwise;
                }
            }

            let mut new_set = HashSet::new();
            new_set.insert(first);
            new_set.insert(second);

            clusters.push(new_set);
        }

        clusters = merge_clusters(clusters);

        Cluster { clusters }
    }
}

fn merge_clusters<'a>(clusters: Vec<HashSet<&'a String>>) -> Vec<HashSet<&'a String>> {
    let mut result = Vec::new();

    for mut cluster in clusters {
        let mut c_index = Vec::new();
        for (i, c) in result.iter().enumerate() {
            if !cluster.is_disjoint(c) {
                c_index.push(i)
            }
        }

        for i in c_index.iter().rev() {
            for elem in result.remove(*i) {
                cluster.insert(elem);
            }
        }

        result.push(cluster);
    }

    result
}

pub trait ClusterAlgo: Sized + Sync {
    fn from_matches(matches: &ArgMatches) -> Self;
    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool;

    fn run(matches: &ArgMatches) {
        run_cluster(&matches, Self::from_matches(&matches))
    }

    fn cluster<'a>(&self, lines: &'a Vec<String>) -> Cluster<'a> {
        let pairs = combinations(lines)
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
    let lines = docs(matches);
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
