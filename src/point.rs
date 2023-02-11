use anyhow::ensure;
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

use crate::field_element::FieldElement;

#[derive(Clone, Debug)]
pub enum Point<const A: i64, const B: i64> {
    Infinity,
    Point(FieldElement, FieldElement),
}

impl<const A: i64, const B: i64> Point<A, B> {
    pub fn new_point(x: FieldElement, y: FieldElement) -> Result<Self, anyhow::Error> {
        ensure!(
            y.clone().pow(2_i32.to_bigint().unwrap())
                == x.clone().pow(3_i32.to_bigint().unwrap())
                    + A * (x.clone())
                    + FieldElement::new(y.prime.clone(), B.to_bigint().unwrap()),
            "({}, {}) is not on the curve",
            x,
            y
        );
        Ok(Point::<A, B>::Point(x, y))
    }

    pub fn new_infinity() -> Self {
        Point::<A, B>::Infinity
    }
}

impl<const A: i64, const B: i64> PartialEq for Point<A, B> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Infinity, Self::Infinity) => true,
            (Self::Point(x1, y1), Self::Point(x2, y2)) => x1 == x2 && y1 == y2,
            _ => false,
        }
    }
}

impl<const A: i64, const B: i64> Add<Point<A, B>> for Point<A, B> {
    type Output = Self;
    fn add(self, other: Point<A, B>) -> Self {
        match (self.clone(), other.clone()) {
            (Self::Infinity, Self::Infinity) => Self::new_infinity(),
            (Self::Infinity, Self::Point(_, _)) => other,
            (Self::Point(_, _), Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 != y2 => {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2))
                if x1 == x2
                    && (y1.num == 0_i32.to_bigint().unwrap()
                        || y2.num == 0_i32.to_bigint().unwrap()) =>
            {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                let s = (FieldElement::new(x1.prime.clone(), 3_i32.to_bigint().unwrap())
                    .mul(x1.clone().pow(2_i32.to_bigint().unwrap()))
                    + A)
                    / (y1.clone() * 2);

                let x3 = s.pow(2_i32.to_bigint().unwrap()) - x1.clone() * 2;
                let y3 = s * (x1 - x3.clone()) - y1;

                Point::<A, B>::new_point(x3, y3).unwrap()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 != x2 => {
                let s = (y2 - y1.clone()) / (x2.clone() - x1.clone());
                let x3 = s.pow(2_i32.to_bigint().unwrap()) - x1.clone() - x2.clone();
                let y3 = s * (x1 - x3.clone()) - y1;
                Self::new_point(x3, y3).unwrap()
            }
            _ => panic!("Invalid points"),
        }
        // if A != rhs.A || self.b != rhs.b {
        //     panic!(
        //         "Points {}, {} are not on the same curve",
        //         self.clone(),
        //         rhs.clone()
        //     );
        // }
        // if self.x == None {
        //     return rhs;
        // }
        // if rhs.x == None {
        //     return self;
        // }
        // if self.x == rhs.x && self.y != rhs.y {
        //     return Self::new(None, None, self.a, self.b);
        // }
        // if self.x != rhs.x {
        //     let s = (rhs.y.clone().unwrap() - self.y.clone().unwrap())
        //         / (rhs.x.clone().unwrap() - self.x.clone().unwrap());
        //     let x = s.clone().pow(2) - self.x.clone().unwrap() - rhs.x.clone().unwrap();
        //     let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
        //     return Self::new(Some(x), Some(y), self.a, self.b);
        // }
        // if self == rhs {
        //     let s = (3_i32.to_bigint().unwrap() * self.x.clone().unwrap().pow(2) + self.a.clone())
        //         / (2_i32.to_bigint().unwrap() * self.y.clone().unwrap());
        //     let x = s.clone().pow(2) - 2_i32.to_bigint().unwrap() * self.x.clone().unwrap();
        //     let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
        //     return Self::new(Some(x), Some(y), self.a.clone(), self.b);
        // }
        // if self == rhs && self.y == Some(0_i32.to_bigint().unwrap() * self.x.clone().unwrap()) {
        //     return Self::new(None, None, self.a, self.b);
        // }
        // Self {
        //     x: self.x,
        //     y: self.y,
        //     a: self.a,
        //     b: self.b,
        // }
    }
}

impl<const A: i64, const B: i64> fmt::Display for Point<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Point::Point(x, y) => {
                write!(f, "Point({},{})_{}_{}", x, y, A, B)
            }
            Point::Infinity => write!(f, "Point(infinity)"),
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

impl<const A: i64, const B: i64> Mul<Point<A, B>> for i64 {
    type Output = Point<A, B>;

    fn mul(self, rhs: Point<A, B>) -> Self::Output {
        let mut result = rhs.clone();
        match rhs.clone() {
            Point::Infinity => return Point::Infinity,
            Point::Point(x, y) => {
                for i in 1..self {
                    result = result.add(rhs.clone());
                }
            }
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigInt;

    use super::*;

    #[test]
    #[should_panic]
    fn test_point_creation_fail() {
        let p = Point::<5, 7>::new_point(
            FieldElement {
                prime: 7_i32.to_bigint().unwrap(),
                num: 1_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: 7_i32.to_bigint().unwrap(),
                num: 1_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();
    }

    #[test]
    fn test_point_creation() {
        let p = Point::<0, 7>::new_point(
            FieldElement {
                prime: 223_i32.to_bigint().unwrap(),
                num: 192_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: 223_i32.to_bigint().unwrap(),
                num: 105_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();
    }

    #[test]
    fn test_ne() {
        let prime = 223_i32.to_bigint().unwrap();
        let p1 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 192_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 105_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        let p2 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 192_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 105_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        let p3 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 1_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 193_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        let inf = Point::<0, 7>::new_infinity();

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, inf);
    }

    // #[test]
    // fn add_two_points_with_the_same_x() {
    //     let p1 = Point::<5, 7>::new_point(-1_i32.to_bigint().unwrap(), -1_i32.to_bigint().unwrap())
    //         .unwrap();
    //     let p2 = Point::<5, 7>::new_point(-1_i32.to_bigint().unwrap(), 1_i32.to_bigint().unwrap())
    //         .unwrap();
    // let inf = Point::<0, 7>::new_infinity();
    //     assert_eq!(p1.clone() + inf.clone(), p1.clone());
    //     assert_eq!(inf.clone() + p2.clone(), p2.clone());
    //     assert_eq!(p1 + p2, inf);
    // }

    #[test]
    fn add_two_points_with_different_x() {
        let prime = 223_i32.to_bigint().unwrap();
        let p1 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 192_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 105_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        let p2 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 17_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 56_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        let p3 = Point::<0, 7>::new_point(
            FieldElement {
                prime: prime.clone(),
                num: 170_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: prime.clone(),
                num: 142_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_point_mul() {
        let p = Point::<0, 7>::new_point(
            FieldElement {
                prime: 223_i32.to_bigint().unwrap(),
                num: 47_i32.to_bigint().unwrap(),
            },
            FieldElement {
                prime: 223_i32.to_bigint().unwrap(),
                num: 71_i32.to_bigint().unwrap(),
            },
        )
        .unwrap();

        assert_eq!(21.mul(p), Point::<0, 7>::new_infinity());
    }
}
