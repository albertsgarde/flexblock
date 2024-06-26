pub fn modulus(lhs: f32, rhs: f32) -> f32 {
    if lhs < 0. {
        (lhs % rhs + rhs) % rhs
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
/// A struct implementing this trait has a additive identity.
pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

/// A struct implementing this trait has a multiplicative identity.
pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.
    }
}

impl One for i32 {
    fn one() -> Self {
        1
    }
}
