use crate::{color::Color, matrix::Matrix, shapes::object::Object, tuple::Tuple};

// pub struct Pattern {
//     pattern_type: Option<Box<dyn PatternType>>,
//     transform: Matrix<f64>,
//     a: Color,
//     b: Color,
// }

pub trait Pattern {
    fn transform(&self) -> &Matrix<f64>;

    fn pattern_at_shape(&self, shape: &Object, world_point: Tuple) -> Color;
}

#[derive(Debug, Clone)]
struct StripePattern {
    transform: Matrix<f64>,
    a: Color,
    b: Color,
}
impl Pattern for StripePattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }
    fn pattern_at_shape(&self, shape: &Object, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        if pattern_point.z.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}
