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

        let split: Vec<&str> = line.split("d").collect();
        let num_rolls: u32 = split[0].parse().unwrap(); 
        let num_sides: u32 = split[1].parse().unwrap();
        
        let rolls: Vec<u32> = (0..num_rolls)
                        .map(|_| rng.gen_range(1, num_sides + 1))
                        .collect();

        println!("{}: {:?}", rolls.iter().sum::<u32>(), rolls);
    }
}

//let rolls_as_string: String = rolls.iter()
//                .map(|roll| roll.to_string())
//                .collect::<Vec<String>>()
//                .join(" ");
