/// Finds the mathematical modulus.
/// That is, the modulus is always positive.
pub fn modulus(lhs: i32, rhs: i32) -> u32 {
    if lhs < 0 {
        (lhs % rhs + rhs) as u32
    } else {
        (lhs % rhs) as u32
    }
}

/// Performs integer division.
/// Always rounds towards negative infinity.
pub fn integer_division(lhs: i32, rhs: i32) -> i32 {
    if lhs < 0 {
        (lhs + 1) / rhs - 1
    } else {
        lhs / rhs
    }
}
