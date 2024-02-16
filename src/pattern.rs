use itertools::Itertools;
use rayon::prelude::*;

use crate::organism::Organism;

#[derive(Debug)]
pub struct Pattern {
    pub vals: Vec<usize>,
}

impl Pattern {
    pub fn new(arg: &String) -> Pattern {
        Pattern {
            vals: arg.split(",").map(|x| x.parse::<usize>().unwrap()).collect(),
        }
    }
    pub fn test(&self, org: &Organism) -> usize {
        org.vals
            .iter()
            .combinations(self.vals.len())
            .par_bridge()
            .filter(|sub| {
                let mut sorted = sub.to_vec();
                sorted.sort();
                let relative = sub
                    .iter()
                    .map(|x| sorted.iter().position(|y| x == y).unwrap())
                    .collect::<Vec<usize>>();

                relative.iter()
                    .zip(self.vals.iter())
                    .par_bridge()
                    .all(|(x, y)| x == y)
            })
            .count()
    }
}
