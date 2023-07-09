pub mod helpers {
    pub fn digit(n: usize) -> usize {
        let mut n = n;
        let mut i = 1;
        let mut digits = 0;
        let mut num = 0;
        let mut result = 0;

        while digits < n {
            num += 1;
            let mut temp = num;
            let mut temp_digits = 0;

            while temp > 0 {
                temp /= 10;
                temp_digits += 1;
            }

            digits += temp_digits;
            i += 1;
        }

        let mut temp = num;
        let mut temp_digits = 0;

        while temp > 0 {
            temp /= 10;
            temp_digits += 1;
        }

        result = num / 10_usize.pow((digits - n) as u32);

        result % 10
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::digit;

    #[test]
    fn test_digit() {
        let result = digit(12);
        let expected = 1;

        assert_eq!(result, expected);
    }
}
