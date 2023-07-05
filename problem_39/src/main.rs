/*
 * If p is the perimeter of a right angle triangle with integral length sides, {a, b, c}, there are exactly three solutions for p = 120.
 * {20, 48, 52}, {24, 45, 51}, {30, 40, 50}
 * For which value of p ≤ 1000, is the number of solutions maximised?
 */

use problem_39::helpers::*;
fn main() {
    let mut count = 0;
    let mut max_num_of_sols = 0;
    let mut max_p = 0;
    for p in 1..=1000 {
        let triples = pythagorean_triples(p);
        if triples.len() > 0 {
            if max_num_of_sols < triples.len() {
                max_num_of_sols = triples.len();
                max_p = p;
            }
        }
    }
    println!("p: {}, num of sols: {}", max_p, max_num_of_sols);
}