use super::ClusterAlgo;
use clap::ArgMatches;
use strsim::osa_distance;

pub struct OSA {
    threshold: usize,
}
impl ClusterAlgo for OSA {
    fn from_matches(matches: &ArgMatches) -> Self {
        let threshold = matches
            .value_of("threshold")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        OSA { threshold }
    }

    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool {
        let sim = osa_distance(first, second);
        sim < self.threshold
    }
}
