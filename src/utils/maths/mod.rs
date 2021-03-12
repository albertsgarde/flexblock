pub mod ray;

/// Finds the mathematical modulus.
/// That is, the modulus is always positive.
pub fn modulus(lhs: f32, rhs: f32) -> f32 {
    if lhs < 0. {
        lhs % rhs + rhs
    } else {
        lhs % rhs
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
