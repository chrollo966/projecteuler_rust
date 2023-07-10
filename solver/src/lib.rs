extern crate permutohedron;

pub mod problem_41 {
    use permutohedron::Heap;

    fn is_prime(n: usize) -> bool {
        match n {
            0 | 1 => false,
            2 => true,
            _ if n % 2 == 0 => false,
            _ => {
                let root = (n as f64).sqrt() as usize;
                !(3..=root).step_by(2).any(|i| n % i == 0)
            }
        }
    }

    pub fn pandigital_primes(n: usize) -> Vec<usize> {
        let mut digits = (1..=n).collect::<Vec<usize>>();
        let permutations = Heap::new(&mut digits);
        let mut primes = Vec::new();

        for permutation in permutations {
            let number = permutation.iter().fold(0, |acc, &x| acc * 10 + x);
            if is_prime(number) {
                primes.push(number);
            }
        }

        primes
    }


}

#[cfg(test)]
mod tests {
    use super::problem_41::*;

    #[test]
    fn test_pandigital_primes() {
        let result = pandigital_primes(4);
        let expected = vec![4231, 4213, 4132, 4123, 3413, 3241, 2341, 2314, 2143, 2134];
        assert_eq!(result, expected);
    }
}
