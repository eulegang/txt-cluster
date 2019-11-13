use super::ClusterAlgo;
use clap::ArgMatches;
use strsim::{damerau_levenshtein, levenshtein};

pub struct Levenshtein {
    threshold: usize,
    damerau: bool,
}

impl ClusterAlgo for Levenshtein {
    fn from_matches(matches: &ArgMatches) -> Self {
        let threshold = matches
            .value_of("threshold")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let damerau = matches.is_present("damerau");

        Levenshtein { threshold, damerau }
    }

    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool {
        let sim = if self.damerau {
            damerau_levenshtein(first, second)
        } else {
            levenshtein(first, second)
        };

        sim < self.threshold
    }
}
