pub mod helpers {
    pub fn rotate_digits(num: usize) -> Vec<usize> {
        let num_str = num.to_string();
        let mut rotated_nums = Vec::new();

        for i in 0..num_str.len() {
            let rotated = format!("{}{}", &num_str[i..], &num_str[..i]);
            rotated_nums.push(rotated.parse::<usize>().unwrap());
        }

        rotated_nums
    }

    fn is_prime(n: usize) -> bool {
        let root = (n as f64).sqrt() as usize;

        for i in 2..=root {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    pub fn all_prime(vec: Vec<usize>) -> bool {
        for item in vec {
            if is_prime(item) == false {
                return false;
            }
        }
        
        true
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::*;

    #[test]
    fn it_works() {
        let result = rotate_digits(197);
        let expected = vec![197, 971, 719];
        assert_eq!(result, expected);
    }

    #[test]
    fn primality_test() {
        let result = is_prime(31);
        let expected = true;
        assert_eq!(result, expected);

        let result2 = is_prime(0);
        assert!(result2);

        let result3 = is_prime(24);
        let expected3 = false;
        assert_eq!(result3, expected3);
    }
}
