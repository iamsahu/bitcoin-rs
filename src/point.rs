use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt;
use std::ops;

#[derive(Clone, Debug, Eq)]
pub struct Point {
    x: Option<BigInt>, // Allows us to represent the point at infinity. Option lets us represent the possibility of a value or the possibility of no value.
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt,
}

impl Point {
    pub fn new(x: Option<BigInt>, y: Option<BigInt>, a: BigInt, b: BigInt) -> Self {
        match (x.clone(), y.clone()) {
            (Some(x_num), Some(y_num)) => {
                if y_num.pow(2) != x_num.pow(3) + a.clone() * x_num.clone() + b.clone() {
                    panic!("({}, {}) is not on the curve", x_num, y_num);
                }
            }

            (None, Some(y_num)) => {
                panic!("Not valid!");
            }
            (Some(x_num), None) => {
                panic!("Not valid!");
            }
            (None, None) => {}
        }
        Self { x, y, a, b }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }
}

impl ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.a != rhs.a || self.b != rhs.b {
            panic!(
                "Points {}, {} are not on the same curve",
                self.clone(),
                rhs.clone()
            );
        }
        if self.x == None {
            return rhs;
        }
        if rhs.x == None {
            return self;
        }
        if self.x == rhs.x && self.y != rhs.y {
            return Self::new(None, None, self.a, self.b);
        }
        if self.x != rhs.x {
            let s = (rhs.y.clone().unwrap() - self.y.clone().unwrap())
                / (rhs.x.clone().unwrap() - self.x.clone().unwrap());
            let x = s.clone().pow(2) - self.x.clone().unwrap() - rhs.x.clone().unwrap();
            let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
            return Self::new(Some(x), Some(y), self.a, self.b);
        }
        if self == rhs {
            let s = (3_i32.to_bigint().unwrap() * self.x.clone().unwrap().pow(2) + self.a.clone())
                / (2_i32.to_bigint().unwrap() * self.y.clone().unwrap());
            let x = s.clone().pow(2) - 2_i32.to_bigint().unwrap() * self.x.clone().unwrap();
            let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
            return Self::new(Some(x), Some(y), self.a.clone(), self.b);
        }
        if self == rhs && self.y == Some(0_i32.to_bigint().unwrap() * self.x.clone().unwrap()) {
            return Self::new(None, None, self.a, self.b);
        }
        Self {
            x: self.x,
            y: self.y,
            a: self.a,
            b: self.b,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.x.clone(), self.y.clone()) {
            (Some(x_num), Some(y_num)) => {
                write!(f, "Point({},{})_{}_{}", x_num, y_num, self.a, self.b)
            }
            (None, None) => write!(f, "Point(infinity)_{}_{}", self.a, self.b),
            _ => {
                panic!("This shouldn't happen");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_point() {
        let x = Some(-1_i32.to_bigint().unwrap());
        let y = Some(-2_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p = Point::new(x, y, a, b);
    }

    #[test]
    fn test_not_equal() {
        let x1 = Some(18_i32.to_bigint().unwrap());
        let y1 = Some(77_i32.to_bigint().unwrap());
        let x2 = Some(-1_i32.to_bigint().unwrap());
        let y2 = Some(-1_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p1 = Point::new(x1, y1, a.clone(), b.clone());
        let p2 = Point::new(x2, y2, a, b);
        assert_ne!(p1, p2);
    }

    #[test]
    fn add_two_points_with_the_same_x() {
        let p1_x = Some(-1_i32.to_bigint().unwrap());
        let p1_y = Some(-1_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(1_i32.to_bigint().unwrap());
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());
        let inf = Point::new(None, None, a.clone(), b.clone());

        assert_eq!(p1.clone() + inf.clone(), p1);
        assert_eq!(inf.clone() + p2.clone(), p2);
        assert_eq!(p1 + p2, inf);
    }
    #[test]
    fn add_two_points_with_different_x() {
        let p1_x = Some(2_i32.to_bigint().unwrap());
        let p1_y = Some(5_i32.to_bigint().unwrap());
        let a = 5_i32.to_bigint().unwrap();
        let b = 7_i32.to_bigint().unwrap();
        let p2_x = Some(-1_i32.to_bigint().unwrap());
        let p2_y = Some(-1_i32.to_bigint().unwrap());
        let p3_x = Some(3_i32.to_bigint().unwrap());
        let p3_y = Some(-7_i32.to_bigint().unwrap());
        let p1 = Point::new(p1_x, p1_y, a.clone(), b.clone());
        let p2 = Point::new(p2_x, p2_y, a.clone(), b.clone());
        let p3 = Point::new(p3_x, p3_y, a.clone(), b.clone());

        assert_eq!(p1 + p2, p3);
    }
}
