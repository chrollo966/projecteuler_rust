/*
 * 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
 * Find the sum of all numbers which are equal to the sum of the factorial of their digits.
 * Note: As 1! = 1 and 2! = 2 are not sums they are not included.
 */

use problem_34::helper_functions::{digit_fact_values, num_to_digits};

fn main() {
    // digit_facts == [1!, 2!, ..., 9!]
    let digit_facts = digit_fact_values();
    println!("{:?}", digit_facts);

    let mut factorions = Vec::new();

    // check that original == factorial sum from 1 to 1000
    for i in 1..1000000 {

        let i_digits = num_to_digits(i); // 145 -> [1, 4, 5]
        let i_digits_for_calc: Vec<_> = i_digits.iter().map(|&x| {
            if x >= 1 {
                x - 1
            } else {
                x
            }
        }).collect(); // [1, 4, 5] -> [0, 3, 4]

        let mut fact_sum = 0;

        for item in i_digits_for_calc {
            fact_sum += digit_facts[item]; // 1! + 4! + 5!
        }

        if fact_sum == i {
            factorions.push(i);
            println!("{} = {}", fact_sum, i);
        }
    }
    
    let factorions_sum_but_1_2 = factorions.iter().fold(0, |acc, &x| acc + x) - 3;
    println!("{:?}", factorions);
    println!("{}", factorions_sum_but_1_2)
}
