use num_bigint::BigInt;
use num_traits::{One, Zero, Num};
use std::rc::Rc;
use lista2::{FiniteField, FieldElement};

#[test]
fn test_secp256k1_prime_field() {
    println!("--- Crypto Test: F_p (256-bit secp256k1) ---");
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::from_str_radix(p_hex, 16).unwrap();

    let field = FiniteField::new_prime_field(p.clone());

    let a_hex = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
    
    let a = FieldElement::from_hex_str(a_hex, field.clone());

    // Inwersja
    let inv_a = a.clone().inverse();
    let unity = a.clone() * inv_a;
    assert_eq!(unity.coeffs[0], BigInt::one(), "Inwersja 256-bitowa nieudana!");

    // Fermat
    let p_minus_1 = &p - BigInt::one();
    let fermat = a.pow(&p_minus_1);
    assert_eq!(fermat.coeffs[0], BigInt::one(), "Fermat failed for 256-bit!");
}

#[test]
fn test_binary_field_aes() {
    println!("--- Crypto Test: F_{{2^8}} (AES) ---");
    let p = BigInt::from(2);
    let modulus_coeffs = vec![1, 1, 0, 1, 1, 0, 0, 0, 1]
        .into_iter().map(BigInt::from).collect();
    let aes_field = FiniteField::new_extension_field(p, modulus_coeffs);

    let a = FieldElement::from_bit_representation(BigInt::from(0x57), aes_field.clone());
    let b = FieldElement::from_bit_representation(BigInt::from(0x83), aes_field.clone());

    let result = a * b;
    let result_int = result.to_bit_representation();

    assert_eq!(result_int, BigInt::from(0xC1), "Błąd mnożenia w ciele AES F_2^8");
}

fn get_aes_field() -> Rc<FiniteField> {
    let p = BigInt::from(2);
    let modulus_coeffs = vec![1, 1, 0, 1, 1, 0, 0, 0, 1]
        .into_iter().map(BigInt::from).collect();
    FiniteField::new_extension_field(p, modulus_coeffs)
}

fn get_secp_field() -> Rc<FiniteField> {
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::from_str_radix(p_hex, 16).unwrap();
    FiniteField::new_prime_field(p)
}

#[test]
fn test_exponentiation_edge_cases() {
    let field = get_secp_field();
    let a = FieldElement::from_hex_str("DEADBEEF", field.clone());

    let pow_zero = a.pow(&BigInt::zero());
    assert_eq!(pow_zero.coeffs[0], BigInt::one(), "a^0 powinno być 1");

    let pow_one = a.pow(&BigInt::one());
    assert_eq!(pow_one.coeffs, a.coeffs, "a^1 powinno być a");

    let pow_two = a.pow(&BigInt::from(2));
    let mul_self = a.clone() * a.clone();
    assert_eq!(pow_two.coeffs, mul_self.coeffs, "a^2 powinno być równe a * a");
}

#[test]
fn test_algebraic_properties_distributivity() {
    let field = get_secp_field();
    let a = FieldElement::from_hex_str("12345", field.clone());
    let b = FieldElement::from_hex_str("ABCDE", field.clone());
    let c = FieldElement::from_hex_str("67890", field.clone());

    let lhs = a.clone() * (b.clone() + c.clone());
    let rhs = (a.clone() * b.clone()) + (a.clone() * c.clone());

    assert_eq!(lhs.coeffs, rhs.coeffs, "Prawo rozdzielności nie działa!");
}

#[test]
fn test_division_large_numbers() {
    let field = get_secp_field();
    let a = FieldElement::from_hex_str("11223344556677889900", field.clone());
    let b = FieldElement::from_hex_str("AABBCCDDEEFF", field.clone());

    let div_result = a.clone() / b.clone();
    let check = div_result * b.clone();

    assert_eq!(check.coeffs, a.coeffs, "Dzielenie nie jest odwrotnością mnożenia!");
}

#[test]
fn test_aes_inversion_known_vector() {
    let field = get_aes_field();
    let a = FieldElement::from_bit_representation(BigInt::from(0x53), field.clone());
    
    let inv_a = a.inverse();
    let inv_val = inv_a.to_bit_representation();

    assert_eq!(inv_val, BigInt::from(0xCA), "Błędna inwersja w AES");
}

#[test]
fn test_freshmans_dream_in_binary_field() {
    let field = get_aes_field();
    let a = FieldElement::from_bit_representation(BigInt::from(0x57), field.clone());
    let b = FieldElement::from_bit_representation(BigInt::from(0x83), field.clone());

    let lhs = (a.clone() + b.clone()).pow(&BigInt::from(2));
    let rhs = a.pow(&BigInt::from(2)) + b.pow(&BigInt::from(2));

    assert_eq!(lhs.to_bit_representation(), rhs.to_bit_representation(), 
        "Freshman's Dream nie działa w ciele binarnym!");
}