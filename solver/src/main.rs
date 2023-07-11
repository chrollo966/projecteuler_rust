extern crate permutohedron;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use permutohedron::Heap;

use solver::helpers::*;

fn problem_41() {
    let mut max = 0;
    for i in 1..=9 {
        let primes = pandigital_primes(i);
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
            if is_triangle_word(&word) {
                count += 1;
            }
        }
    }

    println!("{}", count);
    Ok(())
}

fn problem_43() {
    let mut sum = 0;
    let mut digits = (0..=9).collect::<Vec<u64>>();
    let permutations = Heap::new(&mut digits); 

    for permutation in permutations {
        let number = permutation.iter().fold(0, |acc, &x| acc * 10 + x);
        if is_substr_divisible(number) {
            sum += number;
            println!{"{}", number}
        }
    }
    
    println!("{}", sum);
}

fn problem_44() {
    fn generate_pairs() -> Vec<(u64, u64)> {
        let mut sum_diff_pentagonal_pair = Vec::<(u64, u64)>::new();
        let mut pentagonals = Vec::new();

        for i in 1..=10000 {
            pentagonals.push(pentagonal_number(i));
        }

        for i in 0..10000 {
            let pi = pentagonals[i];
            for j in i + 1..10000 {
                let pj = pentagonals[j];
                if sum_diff_pentagonal(pi, pj) {
                    sum_diff_pentagonal_pair.push((pi, pj));
                }
            }
        }

        sum_diff_pentagonal_pair
    }

    let pairs = generate_pairs();
    let diffs = pairs.iter().map(|(pi, pj)| pi.abs_diff(*pj)).collect::<Vec<u64>>();
    println!("{:?}", diffs.iter().min());
}

fn problem_45() {
    let mut i = 286;
    let mut j = 166;
    let mut k = 144;

    loop {
        let ti = triangle_number(i);
        let pj = pentagonal_number(j);
        let hk = hexiagonal_number(k);

        if ti == pj && pj == hk {
            println!("{}", ti);
            break;
        }

        if ti <= pj && ti <= hk {
            i += 1;
        } else if pj <= ti && pj <= hk {
            j += 1;
        } else {
            k += 1;
        }
    }

    println!("{} {} {}", i, j, k);

}

fn problem_46() {
    for i in (3..).step_by(2) {
        if !is_prime(i) {
            let mut found = false;
            let quotient = i / 2;
            let residue = i % 2;
            for s in (1..=quotient).rev() {
                if is_perfect_square(s as u64) {
                    let residue = 2 * (quotient - s) + residue;
                    if is_prime(residue) {
                        break;
                    }
                }

                if s == 1 {
                    found = true;
                }
            }

            if found {
                println!("{}", i);
                break;
            }
        }
    }
}

fn problem_47() {
    let mut i = 1;
    let mut count = 0;
    loop {
        if non_duprecated_prime_factors(i).len() == 4 {
            count += 1;
        } else {
            count = 0;
        }

        if count == 4 {
            println!("{}", i - 3);
            break;
        }

        i += 1;
    }
}
fn main() {
    problem_47();
}