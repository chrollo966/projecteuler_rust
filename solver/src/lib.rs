#[allow(unused_imports, dead_code)]
extern crate permutohedron;

#[allow(dead_code)]
pub mod helpers {
    use permutohedron::Heap;

    pub fn is_prime(n: usize) -> bool {
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

    pub fn is_substr_divisible(n: u64) -> bool {
        let s = n.to_string();
        let divisors = [2, 3, 5, 7, 11, 13, 17];

        if s.len() == 9 {
            for i in 0..=6 {
                let subseq = s[i..=i + 2].parse::<u64>().unwrap();
                if subseq % divisors[i] != 0 {
                    return false;
                }

            }
        }
        
        for i in 1..=7 {
            let subseq = s[i..=i + 2].parse::<u64>().unwrap();
            if subseq % divisors[i - 1] != 0 {
                return false;
            }
        }

        true
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

    pub fn triangle_number(n: usize) -> u64 {
        n as u64 * (n as u64 + 1) / 2
    }

    pub fn pentagonal_number(n: usize) -> u64 {
        n as u64 * (3 * n as u64 - 1) / 2
    }

    pub fn hexiagonal_number(n: usize) -> u64 {
        n as u64 * (2 * n as u64 - 1)
    }

    pub fn is_perfect_square(n: u64) -> bool {
        let root = (n as f64).sqrt() as u64;
        root * root == n
    }

    pub fn is_pentagonal_number(n: u64) -> bool {
        let root = ((24 * n + 1) as f64).sqrt() as u64;
        root * root == 24 * n + 1 && root % 6 == 5
    }

    pub fn is_hexigaonal_number(n: u64) -> bool {
        let root = ((8 * n + 1) as f64).sqrt() as u64;
        root * root == 8 * n + 1 && root % 4 == 3
    }

    pub fn sum_diff_pentagonal(pi: u64, pj: u64) -> bool {
        is_pentagonal_number(pi + pj) && is_pentagonal_number(pi.abs_diff(pj))
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
                while triangle < score as u64 {
                    n += 1;
                    triangle = triangle_number(n);
                }
                triangle == score as u64
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use permutohedron::Heap;
    use super::helpers::*;

    #[test]
    fn test_problem_43() {
        let mut sum = 0;
        let mut digits = (0..=9).collect::<Vec<u64>>();
        let permutations = Heap::new(&mut digits); 
        for permutation in permutations {
            println!("{:?}", permutation);
        }
    }
}
