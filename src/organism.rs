use std::cmp;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;

use crate::pattern::Pattern;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct Organism {
    pub fitness: usize,
    pub vals: Vec<usize>,
}

impl Organism {
    pub fn new(len: usize) -> Organism {
        let mut v: Vec<usize> = (0..len).collect();
        v.shuffle(&mut thread_rng());
        Organism {
            fitness: 0,
            vals: v,
        }
    }
    pub fn cut_and_crossfill(&mut self, a: &Organism, b: &Organism) {
        let len = self.vals.len();
        let mut rng = rand::thread_rng();
        let mut set = HashSet::new();
        let mut cut = rng.gen_range(0..len);
        for i in 0..cut {
            self.vals[i] = a.vals[i];
            set.insert(a.vals[i]);
        }
        for i in 0..len {
            let v = b.vals[i];
            if !set.contains(&v) {
                self.vals[cut] = v;
                set.insert(v);
                cut += 1;
            }
        }
    }
    pub fn flip_and_scan(&mut self, a: &Organism, b: &Organism) {
        let len = self.vals.len();
        let mut rng = rand::thread_rng();
        let mut set = HashSet::new();
        for i in 0..len {
            let coin = rng.gen_range(0..=1);
            let p = match coin {
                0 => a,
                _ => b
            };
            let mut j = i;
            while set.contains(&p.vals[j]) {
                j = (j + 1) % len;
            }

            let v = p.vals[j];
            self.vals[i] = v;
            set.insert(v);
        }
    }
    pub fn mutate_once(&mut self) {
        let len = self.vals.len();
        let mut rng = rand::thread_rng();
        let i = rng.gen_range(0..len);
        let j = rng.gen_range(0..len);

        let tmp = self.vals[i];
        self.vals[i] = self.vals[j];
        self.vals[j] = tmp;
    }
    pub fn from_ancestors(a: &Organism, b: &Organism) -> Organism {
        let len = a.vals.len();
        let mut out = Organism::new(len);

        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1) {
            0 => {
                // Cut and crossfill
                out.cut_and_crossfill(a, b);
            },
            2 => {
                // Cut and pattern
                /* TODO */
            },
            1 => {
                // Flip and scan
                out.flip_and_scan(a, b);
            }
            _ => { panic!("Invalid RNG value"); }
        }

        while rng.gen_range(0.0..=1.0) > 0.2 {
            out.mutate_once();
        }
        out
    }
    pub fn evaluate(&mut self, p: &Pattern) {
        self.fitness = p.test(self);
    }
    pub fn print(&self) {
        print!("\x1b[2J\x1b[48;2;255;255;255m");
        let len = self.vals.len();
        for (i, x) in self.vals.iter().enumerate() {
            print!("\x1b[{:?};{:?}H  ", x + 1, (2 * (len - i)) + 1);
        }
        println!("\x1b[0m");
    }
}

impl Ord for Organism {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}
