pub mod helpers {
    pub fn is_prime(n: usize) -> bool {
        if n == 0 || n == 1 {
            return false;
        }
        
        let root = (n as f64).sqrt() as usize;

        for i in 2..=root {
            if n % i == 0 {
                return false;
            }
        }

        true
    }

    pub fn trancate_left(num: usize) -> Vec<usize> {
        let num_str = num.to_string();
        let mut trancated_nums = Vec::new();

        for i in 0..num_str.len() {
            let trancated = &num_str[i..];
            trancated_nums.push(trancated.parse::<usize>().unwrap());
        }

        trancated_nums
    }

    pub fn trancate_right(num: usize) -> Vec<usize> {
        let num_str = num.to_string();
        let mut trancated_nums = Vec::new();

        for i in 0..num_str.len() {
            let trancated = &num_str[..num_str.len() - i];
            trancated_nums.push(trancated.parse::<usize>().unwrap());
        }

        trancated_nums
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
        let result = trancate_left(3797);
        let expected = vec![3797, 797, 97, 7];
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