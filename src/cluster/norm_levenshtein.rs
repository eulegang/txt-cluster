use super::ClusterAlgo;
use clap::ArgMatches;
use strsim::{normalized_damerau_levenshtein, normalized_levenshtein};

pub struct NormLevenshtein {
    ratio: f64,
    damerau: bool,
}

impl ClusterAlgo for NormLevenshtein {
    fn from_matches(matches: &ArgMatches) -> Self {
        let ratio = matches.value_of("ratio").unwrap().parse::<f64>().unwrap();
        let damerau = matches.is_present("damerau");

        NormLevenshtein { ratio, damerau }
    }

    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool {
        let sim = if self.damerau {
            normalized_damerau_levenshtein(first, second)
        } else {
            normalized_levenshtein(first, second)
        };

        sim > self.ratio
    }
}
