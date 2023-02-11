use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt;
use std::ops;
use std::ops::Mul;

#[derive(Debug, Clone, Eq)]
pub struct FieldElement {
    pub prime: BigInt,
    pub num: BigInt,
}

impl FieldElement {
    pub fn new(prime: BigInt, num: BigInt) -> Self {
        if num < 0_i32.to_bigint().unwrap() || num >= prime {
            panic!("num {} must be between 0 and {}", num, prime - 1);
        }
        Self { prime, num }
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        let num = self.num.modpow(
            &exponent.modpow(&1_i32.to_bigint().unwrap(), &(self.prime.clone() - 1)), // Forcing out of being negative
            &self.prime,
        );
        Self {
            prime: self.prime.clone(),
            num,
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.prime == other.prime && self.num == other.num
    }
}

impl ops::Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add elements of different primes");
        }
        let num = (self.num + other.num) % self.prime.clone();
        Self {
            prime: self.prime,
            num,
        }
    }
}

impl ops::Add<i64> for FieldElement {
    type Output = FieldElement;

    fn add(self, other: i64) -> FieldElement {
        let num = (other.to_bigint().unwrap() + self.num) % self.prime.clone();
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add elements of different primes");
        }
        let num = (self.num - other.num) % self.prime.clone();
        if num < 0_i32.to_bigint().unwrap() {
            return Self {
                num: num + self.prime.clone(),
                prime: self.prime,
            };
        } else {
            return Self {
                prime: self.prime,
                num,
            };
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add elements of different primes");
        }
        let num = (self.num * other.num) % self.prime.clone();
        Self {
            prime: self.prime,
            num,
        }
    }
}

impl Mul<FieldElement> for i64 {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        let num = (self.to_bigint().unwrap() * other.num) % other.prime.clone();
        FieldElement {
            prime: other.prime,
            num,
        }
    }
}

impl Mul<i64> for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: i64) -> FieldElement {
        let num = (other.to_bigint().unwrap() * self.num) % self.prime.clone();
        FieldElement {
            prime: self.prime,
            num,
        }
    }
}

impl ops::Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add elements of different primes");
        }
        // Fermat's little theorem
        let num = (self.num
            * other.num.modpow(
                &(self.prime.clone() - 2_i32.to_bigint().unwrap()),
                &self.prime,
            ))
            % self.prime.clone();
        Self {
            prime: self.prime,
            num,
        }
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.num.clone(), self.prime.clone()) {
            (num, prime) => {
                write!(f, "FieldElement({},{})", num, prime)
            }
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
    fn test_new() {
        let fe = FieldElement::new(5_i32.to_bigint().unwrap(), 3_i32.to_bigint().unwrap());
        assert_eq!(fe.prime, 5_i32.to_bigint().unwrap());
        assert_eq!(fe.num, 3_i32.to_bigint().unwrap());
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        FieldElement::new(5_i32.to_bigint().unwrap(), 5_i32.to_bigint().unwrap());
    }

    #[test]
    fn add_two_field_elements() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 7_i32.to_bigint().unwrap();
        let b_num = 12_i32.to_bigint().unwrap();
        let c_num = 6_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime.clone(), b_num);
        let c = FieldElement::new(prime, c_num);

        assert_eq!(a + b, c);
    }

    #[test]
    fn sub_two_field_elements() {
        let prime = 19_i32.to_bigint().unwrap();
        let a_num = 6_i32.to_bigint().unwrap();
        let b_num = 13_i32.to_bigint().unwrap();
        let c_num = 12_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime.clone(), b_num);
        let c = FieldElement::new(prime, c_num);

        assert_eq!(a - b, c);
    }

    #[test]
    fn mul_two_field_elements() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 3_i32.to_bigint().unwrap();
        let b_num = 12_i32.to_bigint().unwrap();
        let c_num = 10_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime.clone(), b_num);
        let c = FieldElement::new(prime, c_num);

        assert_eq!(a * b, c);
    }

    #[test]
    fn divide_two_field_elements() {
        let prime = 19_i32.to_bigint().unwrap();
        let a_num = 2_i32.to_bigint().unwrap();
        let b_num = 7_i32.to_bigint().unwrap();
        let c_num = 3_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime.clone(), b_num);
        let c = FieldElement::new(prime, c_num);

        assert_eq!(a / b, c);
    }

    #[test]
    fn power_a_field_element_to_a_positive_exponent() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 3_i32.to_bigint().unwrap();
        let b_num = 1_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime, b_num);
        let exponent = 3_i32.to_bigint().unwrap();

        assert_eq!(a.pow(exponent), b);
    }

    #[test]
    fn power_a_field_element_to_a_negative_exponent() {
        let prime = 13_i32.to_bigint().unwrap();
        let a_num = 7_i32.to_bigint().unwrap();
        let b_num = 8_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime, b_num);
        let exponent = -3_i32.to_bigint().unwrap();

        assert_eq!(a.pow(exponent), b);
    }
}
