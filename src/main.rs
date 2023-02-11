mod field_element;
mod point;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;
    use num_bigint::ToBigInt;

    use crate::field_element;
    use crate::point;

    #[test]
    fn test_on_curve() {
        let prime: BigInt = 223_i32.to_bigint().unwrap();
        let a: field_element::FieldElement =
            field_element::FieldElement::new(prime.clone(), 0_i32.to_bigint().unwrap());
        let b: field_element::FieldElement =
            field_element::FieldElement::new(prime.clone(), 7_i32.to_bigint().unwrap());
        let valid_points = [
            point::Point::<0, 7>::new_point(
                Some(field_element::FieldElement::new(prime.clone(), 192_i32.to_bigint().unwrap())),
                Some(field_element::FieldElement::new(prime.clone(), 105_i32.to_bigint().unwrap())),
            ),
        ] 
    }
}
