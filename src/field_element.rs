use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt;
use std::ops;

#[derive(Debug, Eq)]
pub struct FieldElement {
    prime: BigInt,
    num: BigInt,
}

impl FieldElement {
    pub fn new(prime: BigInt, num: BigInt) -> Self {
        if num < 0_i32.to_bigint().unwrap() || num >= prime {
            panic!("num {} must be between 0 and {}", num, prime - 1);
        }
        Self { prime, num }
    }

    pub fn pow(&self, exponent: &BigInt) -> Self {
        let num = self.num.modpow(exponent, &self.prime);
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

impl ops::Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add elements of different primes");
        }
        let num = (self.num - other.num) % self.prime.clone();
        Self {
            prime: self.prime,
            num,
        }
    }
}

impl ops::Mul for FieldElement {
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
        let prime = 17_i32.to_bigint().unwrap();
        let a_num = 6_i32.to_bigint().unwrap();
        let b_num = 13_i32.to_bigint().unwrap();
        let c_num = 12_i32.to_bigint().unwrap();
        let a = FieldElement::new(prime.clone(), a_num);
        let b = FieldElement::new(prime.clone(), b_num);
        let c = FieldElement::new(prime, c_num);

        assert_eq!(a * b, c);
    }
}
