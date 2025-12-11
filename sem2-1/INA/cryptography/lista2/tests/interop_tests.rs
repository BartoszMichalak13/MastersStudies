use num_bigint::BigInt;
use std::rc::Rc;
use lista2::{FiniteField, FieldElement, EllipticCurve, ECPoint, FieldSerde}; 
// Pamiętaj o imporcie trait 'FieldSerde'!

fn get_secp_field() -> Rc<FiniteField> {
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::parse_bytes(p_hex.as_bytes(), 16).unwrap();
    FiniteField::new_prime_field(p)
}

fn get_binary_field() -> Rc<FiniteField> {
    let p = BigInt::from(2);
    // x^4 + x + 1
    let modulus = vec![1, 1, 0, 0, 1].into_iter().map(BigInt::from).collect();
    FiniteField::new_extension_field(p, modulus)
}

#[test]
fn test_field_element_serialization_prime() {
    println!("--- Test 4: Serialization Round-Trip (F_p) ---");
    let field = get_secp_field();
    
    // Duża liczba
    let val_hex = "DEADBEEF12345678";
    let elem = FieldElement::from_hex_str(val_hex, field.clone());

    // 1. Test HEX
    let s_hex = elem.to_hex();
    println!("Hex: {}", s_hex);
    let recovered_hex = FieldElement::from_hex(&s_hex, field.clone());
    assert_eq!(elem.coeffs, recovered_hex.coeffs, "Hex round-trip failed");

    // 2. Test DECIMAL
    let s_dec = elem.to_decimal();
    println!("Dec: {}", s_dec);
    let recovered_dec = FieldElement::from_decimal(&s_dec, field.clone());
    assert_eq!(elem.coeffs, recovered_dec.coeffs, "Decimal round-trip failed");

    // 3. Test BASE64
    let s_b64 = elem.to_base64();
    println!("B64: {}", s_b64);
    let recovered_b64 = FieldElement::from_base64(&s_b64, field.clone());
    assert_eq!(elem.coeffs, recovered_b64.coeffs, "Base64 round-trip failed");
}

#[test]
fn test_field_element_serialization_extension() {
    println!("--- Test 4: Serialization Round-Trip (F_2^4) ---");
    let field = get_binary_field();
    
    // Wielomian: x^3 + x + 1 -> coeffs [1, 1, 0, 1]
    let coeffs = vec![BigInt::from(1), BigInt::from(1), BigInt::from(0), BigInt::from(1)];
    let elem = FieldElement::new(coeffs, field.clone());

    // 1. Test HEX (oczekujemy formatu tablicowego [1, 1, 0, 1])
    let s_hex = elem.to_hex();
    println!("Poly Hex: {}", s_hex);
    let recovered_hex = FieldElement::from_hex(&s_hex, field.clone());
    assert_eq!(elem.coeffs, recovered_hex.coeffs, "Poly Hex round-trip failed");

    // 2. Test BASE64
    let s_b64 = elem.to_base64();
    println!("Poly B64: {}", s_b64);
    let recovered_b64 = FieldElement::from_base64(&s_b64, field.clone());
    assert_eq!(elem.coeffs, recovered_b64.coeffs, "Poly Base64 round-trip failed");
}

#[test]
fn test_ec_point_serialization() {
    println!("--- Test 4: EC Point Serialization ---");
    // Użyjmy prostej krzywej nad F_17
    let p = BigInt::from(17);
    let field = FiniteField::new_prime_field(p);
    let a = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    // Punkt G = (5, 1)
    let g_x = FieldElement::new(vec![BigInt::from(5)], field.clone());
    let g_y = FieldElement::new(vec![BigInt::from(1)], field.clone());
    let g = curve.point(g_x, g_y).unwrap();

    // 1. Serializacja do HEX
    let serialized_hex = g.serialize("hex");
    println!("Point Hex: {}", serialized_hex); // Oczekujemy "5:1"
    
    let recovered_g = ECPoint::deserialize(&serialized_hex, "hex", curve.clone());
    
    match (g, recovered_g) {
        (ECPoint::Point { x: x1, y: y1, .. }, ECPoint::Point { x: x2, y: y2, .. }) => {
            assert_eq!(x1.coeffs, x2.coeffs);
            assert_eq!(y1.coeffs, y2.coeffs);
        },
        _ => panic!("Serialization failed"),
    }
    
    // 2. Serializacja Infinity
    let inf = curve.infinity();
    let s_inf = inf.serialize("b64");
    println!("Inf serialized: {}", s_inf);
    let rec_inf = ECPoint::deserialize(&s_inf, "b64", curve.clone());
    match rec_inf {
        ECPoint::Infinity { .. } => {}, // OK
        _ => panic!("Infinity serialization failed"),
    }
}