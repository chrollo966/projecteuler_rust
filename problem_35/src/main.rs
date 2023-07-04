/*
 * The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
 * There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
 * How many circular primes are there below one million?
 */

use problem_35::helpers::{ all_prime, rotate_digits };

fn main() {
    let mut count = 0;
    for i in 2..=1000000 {
        let rotated_i = rotate_digits(i);
        if all_prime(rotated_i) == true {
            count += 1;
        }
    }

    println!("{}", count);
}