/*
 * Take the number 192 and multiply it by each of 1, 2, and 3:
 * 192 * 1 = 192
 * 192 * 2 = 384
 * 192 * 3 = 576
 * By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1, 2, 3).
 * The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1, 2, 3, 4, 5).
 * What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1, 2, ... , n) where n > 1?
 */

use problem_38::helpers::*;

fn main() {
    let mut largest_pandigital = 0;
    for n in 1..10000 {
        let mut concatenated_product = String::new();
        let mut m = 1;
        while concatenated_product.len() < 9 {
            concatenated_product.push_str(&(n * m).to_string());
            m += 1;
        }
        if concatenated_product.len() == 9 {
            let concatenated_product = concatenated_product.parse::<u32>().unwrap();
            if is_pandigital(concatenated_product) {
                if concatenated_product > largest_pandigital {
                    largest_pandigital = concatenated_product;
                }
            }
        }
    }
    println!("{}", largest_pandigital);
}