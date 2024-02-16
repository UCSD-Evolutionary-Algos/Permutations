use rayon::prelude::*;

use crate::pattern::Pattern;
use crate::organism::Organism;
use crate::species::Species;

#[derive(Debug)]
pub struct Population {
    species: Vec<Species>,
}

impl Population {
    pub fn new(n_species: usize, orgs_per_species: usize, perm_len: usize) -> Population {
        let mut v = Vec::with_capacity(n_species);
        for _ in 0..n_species {
            v.push(Species::new(orgs_per_species, perm_len));
        }
        Population {
            species: v,
        }
    }
    pub fn evaluate(&mut self, p: &Pattern) {
        self.species.par_iter_mut()
            .for_each(|s| s.evaluate(p));
    }
    pub fn next_gen(&mut self) {
        self.species.par_iter_mut()
            .for_each(|s| s.next_gen());
    }
    pub fn best_performer(&self) -> &Organism {
        let mut out = &self.species[0].organisms[0];
        for s in self.species.iter() {
            let best = s.best_performer();
            if out.fitness < best.fitness {
                out = best;
            }
        }
        out
    }
}
