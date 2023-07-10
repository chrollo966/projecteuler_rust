#[allow(unused_imports, dead_code)]

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

pub mod problem_42 {
    pub fn triangle_number(n: usize) -> usize {
        n * (n + 1) / 2
    }

    pub fn alphabet_to_number(c: char) -> u8 {
        match c {
            'A'..='Z' => c as u8 - 'A' as u8 + 1,
            'a'..='z' => c as u8 - 'a' as u8 + 1,
            _ => panic!("Alphabet only"),
        }
    }

    pub fn is_triangle_word(word: &str) -> bool {
        let score = word.chars().map(alphabet_to_number).sum::<u8>();

        match score {
            1 => true,
            _ => {
                let mut n = 1;
                let mut triangle = triangle_number(n);
                while triangle < score as usize {
                    n += 1;
                    triangle = triangle_number(n);
                }
                triangle == score as usize
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::problem_42::*;

    #[test]
    fn test_alph2num() {
        let result = alphabet_to_number('A');
        let expected = 1;

        assert_eq!(result, expected);
    }
}
