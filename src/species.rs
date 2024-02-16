use rayon::prelude::*;

use crate::pattern::Pattern;
use crate::organism::Organism;

#[derive(Debug)]
pub struct Species {
    pub organisms: Vec<Organism>,
}

impl Species {
    pub fn new(size: usize, perm_len: usize) -> Species {
        let mut v = Vec::with_capacity(size);
        for _ in 0..size {
            v.push(Organism::new(perm_len));
        }
        Species {
            organisms: v,
        }
    }
    pub fn evaluate(&mut self, p: &Pattern) {
        self.organisms.par_iter_mut()
            .for_each(|o| o.evaluate(p));
    }
    pub fn next_gen(&mut self) {
        self.organisms.sort();
        let len = self.organisms.len();
        let mut children = Vec::with_capacity(len);
        // TODO: This is wasteful
        for _ in 0..len {
            children.push(Organism::new(0));
        }
        children.par_iter_mut().for_each(|r| {
            let mut rng = rand::thread_rng();
            // TODO: Investigate genetic diversity
            /*let a = &self.organisms[rng.gen_range((len / 2)..len)];
            let b = &self.organisms[rng.gen_range((len / 2)..len)];*/
            let a = &self.organisms[len - 1];
            let b = &self.organisms[len - 2];
            *r = Organism::from_ancestors(a, b);
        });
        self.organisms = children;
    }
    pub fn best_performer(&self) -> &Organism {
        let mut out = &self.organisms[0];
        for o in self.organisms.iter() {
            if out.fitness < o.fitness {
                out = &o;
            }
        }
        out
    }
}
