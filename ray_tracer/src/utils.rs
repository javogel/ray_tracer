pub type Scalar = f32;
pub const EPSILON: Scalar = 0.00001;

pub fn is_odd(n: usize) -> bool {
    return if n == 0 { false } else { n % 2 != 0 };
}
