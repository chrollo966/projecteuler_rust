pub mod helpers {
    pub fn is_palindromic_str(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        s == s.iter().rev().cloned().collect::<Vec<char>>()
    }

    pub fn both_palindlome(n: usize) -> bool {
        let n_decimal = n.to_string();
        let n_binary = format!("{:b}", n);
        
        is_palindromic_str(n_decimal) && is_palindromic_str(n_binary)
    }
}

#[cfg(test)]
mod tests {
    use super::helpers::*;
}
