use crate::combinations::*;
use crate::utils::run_cluster;
use clap::ArgMatches;
use rayon::prelude::*;
use std::collections::HashSet;

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

        clusters = Cluster::merge(clusters);

        Cluster { clusters }
    }

    fn merge(clusters: Vec<HashSet<&'a String>>) -> Vec<HashSet<&'a String>> {
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
}

impl<'a> IntoIterator for Cluster<'a> {
    type Item = HashSet<&'a String>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.clusters.into_iter()
    }
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
