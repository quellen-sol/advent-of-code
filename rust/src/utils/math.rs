pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs();
    b = b.abs();
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }

    b
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }

    // Dividing first keeps the intermediate value as small as the result.
    (a / gcd(a, b) * b).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, -18), 6);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(-4, 6), 12);
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(0, 0), 0);
        assert_eq!(lcm(i64::MAX, i64::MAX), i64::MAX);
    }
}
