use anyhow::ensure;
use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub enum Point<const A: i64, const B: i64> {
    Infinity,
    Point(i64, i64),
}

impl<const A: i64, const B: i64> Point<A, B> {
    pub fn new_point(x: i64, y: i64) -> Result<Self, anyhow::Error> {
        ensure!(
            y.pow(2) == x.pow(3) + A * x + B,
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
        match (self, other) {
            (Self::Infinity, Self::Infinity) => Self::new_infinity(),
            (Self::Infinity, Self::Point(_, _)) => other,
            (Self::Point(_, _), Self::Infinity) => self,
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 != y2 => {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && (y1 == 0 || y2 == 0) => {
                Self::new_infinity()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 == x2 && y1 == y2 => {
                let s = (3 * x1.pow(2) + A) / (2 * y1);
                let x3 = s.pow(2) - 2 * x1;
                let y3 = s * (x1 - x3) - y1;
                Self::new_point(x3, y3).unwrap()
            }
            (Self::Point(x1, y1), Self::Point(x2, y2)) if x1 != x2 => {
                let s = (y2 - y1) / (x2 - x1);
                let x3 = s.pow(2) - x1 - x2;
                let y3 = s * (x1 - x3) - y1;
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

// impl<const A: i64, const B: i64> fmt::Display for Point<A, B> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match (self.x.clone(), self.y.clone()) {
//             (Some(x_num), Some(y_num)) => {
//                 write!(f, "Point({},{})_{}_{}", x_num, y_num, self.a, self.b)
//             }
//             (None, None) => write!(f, "Point(infinity)_{}_{}", self.a, self.b),
//             _ => {
//                 panic!("This shouldn't happen");
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_point() {
        let p = Point::<5, 7>::new_point(1, 1).unwrap();
    }

    #[test]
    fn test_ne() {
        let p1 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let p2 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let p3 = Point::<5, 7>::new_point(-1, 1).unwrap();
        let inf = Point::<5, 7>::new_infinity();

        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
        assert_ne!(p1, inf);
    }

    #[test]
    fn add_two_points_with_the_same_x() {
        let p1 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let p2 = Point::<5,7>::new_point(-1, 1).unwrap();
        let inf = Point::<5,7>::new_infinity();

        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p2, p2);
        assert_eq!(p1 + p2, inf);
    }
    #[test]
    fn add_two_points_with_different_x() {
        
        let p1 = Point::<5, 7>::new_point(2, 5).unwrap();
        let p2 = Point::<5, 7>::new_point(-1, -1).unwrap();
        let p3 = Point::<5, 7>::new_point(3,-7).unwrap();

        assert_eq!(p1 + p2, p3);
    }
}
