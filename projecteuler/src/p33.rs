pub mod check_dcf {
    pub fn check_dcf() {
        // dcf: digit cancelling fraction
        let mut numer_prod = 1;
        let mut denom_prod = 1;

        for a in 1..8 {
            for d in (a + 1)..9 {
                for c in (d + 1)..10 {
                    if 9 * a * (d - c) == c * (a - d) {
                        // numer = numer * a;
                        // denom = denom * d;
                        println!("a:  {}, b: {}, c: {}, d: {}", a, c, c, d);
                        numer_prod = a * numer_prod;
                        denom_prod = d * denom_prod;
                        println!("denom_prod: {}", denom_prod)
                    }
                }
            }
        }
        println!("The product of denominators is {}",denom_prod / num::integer::gcd(denom_prod, numer_prod));
    }
}