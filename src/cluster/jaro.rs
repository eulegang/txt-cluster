use super::ClusterAlgo;
use clap::ArgMatches;
use strsim::{jaro, jaro_winkler};

pub struct Jaro {
    ratio: f64,
    winkle: bool,
}

impl ClusterAlgo for Jaro {
    fn from_matches(matches: &ArgMatches) -> Self {
        let ratio = matches.value_of("ratio").unwrap().parse::<f64>().unwrap();
        let winkle = matches.is_present("winkler");

        Jaro { ratio, winkle }
    }

    fn accept<'a>(&self, first: &'a String, second: &'a String) -> bool {
        let sim = if self.winkle {
            jaro_winkler(first, second)
        } else {
            jaro(first, second)
        };

        sim > self.ratio
    }
}
