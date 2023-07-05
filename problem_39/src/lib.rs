pub mod helpers {
    fn gcd(m: usize, n: usize) -> usize {
        let mut left = m;
        let mut right = n;
        while right != 0 {
            let temp = right;
            right = left % right;
            left = temp;
        }
        left
    }

    fn is_coprime(n: usize, m: usize) -> bool {
        gcd(n, m) == 1
    }

    fn either_is_even(m: usize, n: usize) -> bool {
        m % 2 == 0 || n % 2 == 0
    }

    fn is_m_n_candidate(m: usize, n: usize) -> bool {
        m > n && n > 0 && is_coprime(m, n) && either_is_even(m, n)
    }

    fn factor_pairs(n: usize) -> Vec<(usize, usize)> {
        let mut pairs = Vec::new();
        for i in 1..=(n as f64).sqrt() as usize {
            if n % i == 0 {
                pairs.push((i, n / i));
            }
        }
        pairs
    }

    pub fn m_n_candidates(p: usize) -> Vec<(usize, usize)> {
        factor_pairs(p).into_iter().map(|(m, n)| (m, n - m)).collect()
    }

    pub fn pythagorean_triples(p: usize) -> Vec<(usize, usize, usize)> {
        if p % 2 != 0 {
            return Vec::new()
        }

        let half_p = p / 2;
        let mut factors: Vec<usize> = Vec::new();
        let mut pythagorean_triples: Vec<(usize, usize, usize)> = Vec::new();

        for i in 1..=(half_p as f64).sqrt() as usize {
            if half_p % i == 0 {
                factors.push(i);
                factors.push(half_p / i)
            }
        }

        for factor in factors {
            m_n_candidates(half_p / factor).into_iter().filter(|(m, n)| is_m_n_candidate(*m, *n)).for_each(|(m, n)| {
                let a = factor * (m * m - n * n);
                let b = factor * (2 * m * n);
                let c = factor * (m * m + n * n);
                pythagorean_triples.push((a, b, c));
            });
        }

        pythagorean_triples
    }

}

#[cfg(test)]
mod tests {
    use super::helpers::*;

    #[test]
    fn test_pythagorean_triples() {
        let result = pythagorean_triples(120);
        let expected = vec![(20, 48, 52), (24, 45, 51), (30, 40, 50)];

        assert_eq!(result, expected);
    }

}
