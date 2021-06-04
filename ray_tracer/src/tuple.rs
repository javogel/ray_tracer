use std::{ops, fmt};

const EPSILON: Scalar = 0.00001;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TupleType {
    Point,
    Vector,
}

type Scalar = f32;
type TT = TupleType;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
    pub w: TupleType,
}

impl Tuple {
    fn is_a_vector(&self) -> bool {
        match self.w {
            TT::Vector => true,
            TT::Point => false,
        }
    }

    fn addition(&self, b: Tuple) -> Tuple {
        let w = match (&self.w, b.w) {
            (TT::Vector, TT::Point) => TT::Point,
            (TT::Point, TT::Vector) => TT::Point,
            (TT::Vector, TT::Vector) => TT::Vector,
            (TT::Point, TT::Point) => panic!("Two Points cannot be added"),
        };

        Tuple {
            x: self.x + b.x,
            y: self.y + b.y,
            z: self.z + b.z,
            w,
        }
    }

    fn subtraction(&self, b: Tuple) -> Tuple {
        let w = match (&self.w, b.w) {
            (TT::Point, TT::Point) => TT::Vector,
            (TT::Point, TT::Vector) => TT::Point,
            (TT::Vector, TT::Vector) => TT::Vector,
            (TT::Vector, TT::Point) => panic!("Cannot subtract Point from vector"),
        };

        Tuple {
            x: self.x - b.x,
            y: self.y - b.y,
            z: self.z - b.z,
            w,
        }
    }

    fn negate(&self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    fn multiply(&self, s: Scalar) -> Tuple {
        Tuple {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w,
        }
    }

    fn divide(&self, s: Scalar) -> Tuple {
        Tuple {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
            w: self.w,
        }
    }

    fn magnitude(&self) -> f32 {
        match self {
            Tuple {
                x,
                y,
                z,
                w: TT::Vector,
            } => (x.powi(2) + y.powi(2) + z.powi(2)).sqrt(),
            _ => panic!("Magnitude can only be called on Tuples of type Vector"),
        }
    }

    fn normalize(&self) -> Tuple {
        match self.w {
            TT::Vector => {
                let m = self.magnitude();
                self.divide(m)
            }
            _ => panic!("Normalize can only be called on Tuples of type Vector"),
        }
    }

    pub fn dot(&self, b: Tuple) -> f32 {
        match (&self.w, b.w) {
            (TT::Vector, TT::Vector) => self.x * b.x + self.y * b.y + self.z * b.z,
            _ => panic!("Dot product can only be calculated on two Vectors"),
        }
    }

    pub fn cross(&self, b: Tuple) -> Tuple {
        match (&self.w, b.w) {
            (TT::Vector, TT::Vector) => Tuple {
                x: self.y * b.z - self.z * b.y,
                y: self.z * b.x - self.x * b.z,
                z: self.x * b.y - self.y * b.x,
                w: TT::Vector,
            },
            _ => panic!("Dot product can only be calculated on two Vectors"),
        }
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Tuple {
        self.addition(rhs)
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, rhs: Tuple) -> Tuple {
        self.subtraction(rhs)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        self.negate()
    }
}

impl ops::Mul<Scalar> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: Scalar) -> Tuple {
        self.multiply(rhs)
    }
}

impl ops::Div<Scalar> for Tuple {
    type Output = Tuple;

    fn div(self, rhs: Scalar) -> Tuple {
        self.divide(rhs)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Tuple{x, y, z,  w} = self;
        write!(f, "{}({}, {}, {})", w, x, y, z)
    }
}

impl fmt::Display for TupleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_name = match self {
            TT::Vector => "Vector",
            TT::Point => "Point"
        };
        write!(f,"{}", type_name)
    }
}

pub fn is_a_vector(t: Tuple) -> bool {
    t.is_a_vector()
}

pub fn point(x: Scalar, y: Scalar, z: Scalar) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: TT::Point,
    }
}

pub fn vector(x: Scalar, y: Scalar, z: Scalar) -> Tuple {
    Tuple {
        x,
        y,
        z,
        w: TT::Vector,
    }
}

pub fn equal(a: Scalar, b: Scalar) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        false
    }
}

pub fn add(a: Tuple, b: Tuple) -> Tuple {
    a + b
}

pub fn subtract(a: Tuple, b: Tuple) -> Tuple {
    a - b
}

pub fn negate(a: Tuple) -> Tuple {
    -a
}

pub fn multiply(a: Tuple, s: Scalar) -> Tuple {
    a * s
}

pub fn divide(a: Tuple, s: Scalar) -> Tuple {
    a / s
}

pub fn magnitude(a: Tuple) -> f32 {
    a.magnitude()
}

pub fn normalize(a: Tuple) -> Tuple {
    a.normalize()
}

pub fn dot(a: Tuple, b: Tuple) -> f32 {
    a.dot(b)
}

pub fn cross(a: Tuple, b: Tuple) -> Tuple {
    a.cross(b)
}
