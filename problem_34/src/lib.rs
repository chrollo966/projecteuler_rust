pub mod helper_functions {

    pub fn digit_fact_values() -> Vec<usize> {
        let mut product = 1;
        let digit_facts: Vec<_> = (1..=9)
            .scan((), move |_, i| {
                product *= i;
                Some(product)
            })
            .collect();

        digit_facts
    }

    // 145 -> [1, 4, 5]
    pub fn num_to_digits(num: usize) -> Vec<usize> {
        let mut digits = Vec::new();
        let mut n = num;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.reverse();
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(helper_functions::num_to_digits(100), [1, 0, 0]);
    }
}