use crate::utils::*;
use std::ops;

#[derive(Debug, Copy, Clone)]

pub struct Color {
    pub r: Scalar,
    pub b: Scalar,
    pub g: Scalar,
}

impl Color {
    fn add_color(&self, b: Color) -> Color {
        Color {
            r: self.r + b.r,
            g: self.g + b.g,
            b: self.b + b.b,
        }
    }

    fn subtract_color(&self, b: Color) -> Color {
        Color {
            r: self.r - b.r,
            g: self.g - b.g,
            b: self.b - b.b,
        }
    }

    fn multiply_scalar(&self, b: Scalar) -> Color {
        Color {
            r: self.r * b,
            g: self.g * b,
            b: self.b * b,
        }
    }
    fn multiply_color(&self, t: Color) -> Color {
        self.hadamard_product(t)
    }

    fn hadamard_product(&self, b: Color) -> Color {
        Color {
            r: self.r * b.r,
            g: self.g * b.g,
            b: self.b * b.b,
        }
    }
}

impl ops::Mul<Scalar> for Color {
    type Output = Color;

    fn mul(self, rhs: Scalar) -> Color {
        self.multiply_scalar(rhs)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        self.multiply_color(rhs)
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        self.add_color(rhs)
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        self.subtract_color(rhs)
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let e = EPSILON;
        let Color { r, g, b } = self;
        r - other.r < e && g - other.g < e && b - other.b < e
    }
}

pub fn color(r: Scalar, g: Scalar, b: Scalar) -> Color {
    Color { r, g, b }
}

pub fn add(a: Color, b: Color) -> Color {
    a + b
}

pub fn multiply<W, T>(a: W, b: T) -> W::Output
where
    W: std::ops::Mul<T>,
{
    a * b
}

pub fn subtract(a: Color, b: Color) -> Color {
    a - b
}

pub fn black() -> Color {
    color(0., 0., 0.)
}
