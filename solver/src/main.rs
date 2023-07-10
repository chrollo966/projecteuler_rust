use solver::problem_41;

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

fn main() {
    problem_41();
}
