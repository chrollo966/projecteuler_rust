/*
 * The decimal number, 585 = 1001001001_2 (binary), is palindromic in both bases.
 * Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
 * (Please note that the palindromic number, in either base, may not include leading zeros.)
 */

use problem_36::helpers::{ both_palindlome };

fn main() {
    let mut sum: usize = 0;
    for n in 1..1000000 {
        if both_palindlome(n) {
            sum += n;
        }
    }
    println!("{}", sum);
}