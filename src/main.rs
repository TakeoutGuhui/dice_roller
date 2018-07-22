extern crate rand;
extern crate regex;

use std::io;
use rand::{Rng, thread_rng};
use regex::Regex;

fn main() {
    let mut rng = thread_rng();

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("couldn't read line");
        let line = line.trim();

        let re = Regex::new(r"^\d+d\d+$").unwrap();
        if !re.is_match(&line) { 
            println!("not the right format");
            continue;
        }

        let num_split: Vec<u32> = line.split("d")
                        .map(|x| x.parse::<u32>().expect("couldn't parse number"))
                        .collect();

        let (num_rolls, num_sides) = (num_split[0], num_split[1]); 

        let dice_rolls: Vec<u32> = (0..num_rolls)
                        .map(|_| rng.gen_range(1, num_sides + 1))
                        .collect();

        println!("{}: {:?}", dice_rolls.iter().sum::<u32>(), dice_rolls);
    }
