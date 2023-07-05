pub mod helpers {
    pub fn concatenated_product(n: u32, m: u32) -> u32 {
        let mut result = String::new();
        for i in 1..=m {
            result.push_str(&(n * i).to_string());
        }
        result.parse::<u32>().unwrap()
    }

    pub fn is_pandigital(n: u32) -> bool {
        let mut digits = vec![false; 10];
        let mut n = n;
        while n > 0 {
            let digit = n % 10;
            if digit == 0 {
                return false;
            }
            if digits[digit as usize] {
                return false;
            }
            digits[digit as usize] = true;
            n /= 10;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenated_product() {
        assert_eq!(helpers::concatenated_product(192, 3), 192384576);
        assert_eq!(helpers::concatenated_product(9, 5), 918273645);
    }

    #[test]
    fn test_is_pandigital() {
        assert_eq!(helpers::is_pandigital(192384576), true);
        assert_eq!(helpers::is_pandigital(918273645), true);
        assert_eq!(helpers::is_pandigital(192384575), false);
        assert_eq!(helpers::is_pandigital(918273644), false);
    }
}
