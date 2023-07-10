use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use solver::problem_41;
use solver::problem_42;

fn problem_41() {
    let mut max = 0;
    for i in 1..=9 {
        let primes = problem_41::pandigital_primes(i);
        if let Some(max_prime) = primes.iter().max() {
            if max_prime > &max {
                max = *max_prime;
            }
        }
        println!("{}: {:?}", i, max);
    }
}

fn problem_42() -> io::Result<()> {
    let path = Path::new("resources/0042_words.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line = line?;
        let words: Vec<_> = line.split(',').map(|word| word.replace("\"", "")).collect();

        for word in words {
            if problem_42::is_triangle_word(&word) {
                count += 1;
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn main() {
    problem_42().unwrap();
}