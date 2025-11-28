//! Rational arithmetic operations.

/// Computes the greatest common divisor.
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Reduces a fraction to lowest terms.
pub fn reduce(num: i64, den: u64) -> (i64, u64) {
    let g = gcd(num.unsigned_abs(), den);
    (num / g as i64, den / g)
}

/// Adds two rational numbers.
pub fn add(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    let num = n1 * d2 as i64 + n2 * d1 as i64;
    reduce(num, d1 * d2)
}

/// Subtracts first from second rational.
pub fn subtract(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    let num = n2 * d1 as i64 - n1 * d2 as i64;
    reduce(num, d1 * d2)
}

/// Multiplies two rational numbers.
pub fn multiply(n1: i64, d1: u64, n2: i64, d2: u64) -> (i64, u64) {
    reduce(n1 * n2, d1 * d2)
}

/// Divides second by first. Returns None if first is zero.
pub fn divide(n1: i64, d1: u64, n2: i64, d2: u64) -> Option<(i64, u64)> {
    if n1 == 0 {
        return None;
    }
    let sign = if n1 < 0 { -1 } else { 1 };
    let num = n2 * d1 as i64 * sign;
    let den = d2 * n1.unsigned_abs();
    Some(reduce(num, den))
}
