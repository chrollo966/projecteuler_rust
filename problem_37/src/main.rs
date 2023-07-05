/*
 * The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
 * Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
 * NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
 */

use problem_37::helpers::*;

fn main() {
    let mut count = 0;
    let mut sum = 0;
    for n in 10..1000000 {
        if is_prime(n) && count < 11 {
            let left = trancate_left(n);
            let right = trancate_right(n);

            if all_prime(left) && all_prime(right) {
                count += 1;
                sum += n;
                println!("{} is a truncatable prime", n);
            }
        } else if count == 11 {
            break;
        }
    }
    println!("The sum of the truncatable primes is {}", sum);
}
