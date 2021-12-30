use crate::{
    color::{color, Color},
    matrix::{identity, Matrix},
    shapes::object::Object,
    tuple::Tuple,
};

pub trait Pattern: Send + Sync {
    fn transform(&self) -> &Matrix<f64>;

    fn at_point(&self, point: Tuple) -> Color;

    fn at_object(&self, shape: &Object, world_point: Tuple) -> Color {
        let object_point = shape.transform.inverse().unwrap() * world_point;
        let pattern_point = self.transform().inverse().unwrap() * object_point;
        self.at_point(pattern_point)
    }
}

#[derive(Debug, Clone)]
pub struct TestPattern {
    pub transform: Matrix<f64>,
}
impl Pattern for TestPattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    fn at_point(&self, point: Tuple) -> Color {
        color(point.x, point.y, point.z)
    }
}

#[derive(Debug, Clone)]
pub struct StripePattern {
    pub transform: Matrix<f64>,
    pub a: Color,
    pub b: Color,
}
impl Pattern for StripePattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    fn at_point(&self, point: Tuple) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

pub struct GradientPattern {
    pub transform: Matrix<f64>,
    pub a: Color,
    pub b: Color,
}

impl Pattern for GradientPattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    fn at_point(&self, point: Tuple) -> Color {
        let disance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        return self.a + disance * fraction;
    }
}

pub struct RingPattern {
    pub transform: Matrix<f64>,
    pub a: Color,
    pub b: Color,
}

impl Pattern for RingPattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    fn at_point(&self, point: Tuple) -> Color {
        let distance = (point.x.powf(2.) + point.z.powf(2.)).sqrt();
        if distance % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

pub struct CheckerPattern {
    pub transform: Matrix<f64>,
    pub a: Color,
    pub b: Color,
}

impl Pattern for CheckerPattern {
    fn transform(&self) -> &Matrix<f64> {
        &self.transform
    }

    fn at_point(&self, point: Tuple) -> Color {
        let val = (point.x.abs() + point.y.abs() + point.z.abs()).floor();
        if val % 2. == 0. {
            self.a
        } else {
            self.b
        }
    }
}

pub fn test_pattern() -> TestPattern {
    TestPattern {
        transform: identity(),
    }
}

pub fn stripe_pattern(a: Color, b: Color) -> StripePattern {
    StripePattern {
        a,
        b,
        transform: identity(),
    }
}

pub fn gradient_pattern(a: Color, b: Color) -> GradientPattern {
    GradientPattern {
        a,
        b,
        transform: identity(),
    }
}

pub fn ring_pattern(a: Color, b: Color) -> RingPattern {
    RingPattern {
        a,
        b,
        transform: identity(),
    }
}

pub fn checker_pattern(a: Color, b: Color) -> CheckerPattern {
    CheckerPattern {
        a,
        b,
        transform: identity(),
    }
}
