use rand::prelude::SliceRandom;
use std::vec::Vec;

use std::thread;
use num_cpus;

use clap::Parser;


fn prepare_boxes(n_prisoners : usize) -> Vec<usize> {
    let mut rng = rand::rng();
    let mut boxes: Vec<usize> = (0..n_prisoners).collect();

    boxes.shuffle(&mut rng);

    boxes
}

fn find_longest_loop(boxes : &Vec<usize>, seen_numbers : &mut Vec<usize>) -> usize {
    seen_numbers.clear();
    
    let mut longest: usize = 0;

    for loop_start in 0..boxes.len() {
        if seen_numbers.contains(&loop_start){
            continue;
        }
        seen_numbers.push(loop_start);
        
        let mut current_box: usize = loop_start;
        let mut current_length: usize = 1;

        loop {
            let content = boxes[current_box as usize];
            
            if content != loop_start {
                seen_numbers.push(content);
                current_box = content;
                current_length += 1;
            } else {
                break;
            }
        }

        if current_length > longest {
            longest = current_length;
        }
    }

    longest

}

fn worker(n_trials: usize, n_prisoners : usize) -> u32 {
    let mut seen_numbers: Vec<usize> = Vec::with_capacity(n_prisoners);
    let mut n_survived = 0;

    for _ in 0..n_trials {
        let boxes = prepare_boxes(n_prisoners);
        if find_longest_loop(&boxes, &mut seen_numbers) <= n_prisoners / 2 {
            n_survived += 1;
        }
    }
    n_survived
}

fn main() {
    let args = Args::parse();

    let n_prisoners: usize = args.prisoners;
    let n_trials: u32 = args.trials;
    let n_workers: u32 = if args.trials >= (args.workers as u32) {args.workers as u32} else {args.trials};

    if n_trials == 0 {
        panic!("n_trials must be > 0");
    }

    let n_cpus = num_cpus::get() as u32;

    if n_workers == 0 {
        panic!("n_workers must be > 0");
    } else if n_workers > n_cpus {
        println!("WARNING: Using {} workers although only {} CPUs are available. Recommended maximum is {}.", n_workers, n_cpus, n_cpus - 1);
    }

    let n_trials_per_worker = n_trials / n_workers;
    let n_trials_last_worker = n_trials_per_worker + n_trials % n_workers;

    if n_prisoners % 2 != 0 {
        panic!("n_prisoners is not even");
    }

    let mut handles = Vec::new();
    
    for worker_idx in 0..n_workers {
        handles.push(thread::spawn(move || {
            let n_trials_thread = if worker_idx != n_workers - 1 {n_trials_per_worker} else {n_trials_last_worker};

            worker(n_trials_thread as usize, n_prisoners)
        }));
    }

    let mut n_survived = 0;

    for handle in handles{
        n_survived += handle.join().unwrap();
    }
    
    println!("{}", (n_survived as f64)/(n_trials as f64));
}

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Args {
    // Number of prisoners to simulate (must be even)
    #[arg(short, long)]
    prisoners: usize,

    // Number of trials to perform
    #[arg(short, long)]
    trials: u32,

    // Number of workers (threads) to use
    #[arg(short, long, default_value_t = num_cpus::get() - 1)]
    workers: usize,
}
