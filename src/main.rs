use std::env;
use std::process::exit;

mod pattern;
mod organism;
mod species;
mod population;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: permutations [permutation length] [comma-separated pattern]");
        exit(1);
    }

    let len = args[1].parse::<usize>().unwrap();
    let p = pattern::Pattern::new(&args[2]);
    let mut pop = population::Population::new(1, 100, len);
    for _ in 0..100 {
        pop.evaluate(&p);
        pop.best_performer().print();
        pop.next_gen();
    }
    pop.evaluate(&p);
    println!("{:#?}", pop.best_performer());
}
