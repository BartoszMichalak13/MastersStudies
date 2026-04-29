use num_bigint::BigInt;
use lista2::{
    FiniteField, FieldElement, EllipticCurve,
    SchnorrField, SchnorrCurve, encode_r, SchnorrR
};

// --- Testy Kodowania (Zadanie 4) ---

#[test]
fn test_encoding_examples() {
    println!("--- Test: Schnorr Encoding Rules ---");

    // 1. Encode(17 \in F_65537) = "000011"
    let p1 = BigInt::from(65537);
    let f1 = FiniteField::new_prime_field(p1);
    let val1 = FieldElement::new(vec![BigInt::from(17)], f1);
    assert_eq!(encode_r(&SchnorrR::Field(val1)), "\"000011\"");

    // 2. Encode(x^3 + x^2 + 1 \in F_2^33) = "000000000D"
    let p2 = BigInt::from(2);
    // Symulacja ciała F_2^33 (modulus stopnia 33)
    let mut mod_coeffs = vec![BigInt::from(0); 34]; 
    mod_coeffs[33] = BigInt::from(1);
    let f2 = FiniteField::new_extension_field(p2, mod_coeffs);
    
    // Wartość 13 (0xD)
    let val2 = FieldElement::from_bit_representation(BigInt::from(13), f2);
    assert_eq!(encode_r(&SchnorrR::Field(val2)), "\"000000000D\"");

    // 3. Encode(3x^2 + 16 \in F_17^3) = ["10", "00", "03"]
    // Ciało rozszerzone F_p^k gdzie p=17, k=3
    let p_ext = BigInt::from(17);
    // Modulus musi mieć stopień k=3 (np. x^3)
    let mod_coeffs_ext = vec![BigInt::from(0), BigInt::from(0), BigInt::from(0), BigInt::from(1)];
    let f_ext = FiniteField::new_extension_field(p_ext, mod_coeffs_ext);

    // Wielomian 3x^2 + 0x + 16. Współczynniki w kolejności od x^0: [16, 0, 3]
    let val_ext = FieldElement::new(vec![BigInt::from(16), BigInt::from(0), BigInt::from(3)], f_ext);

    // Sprawdzamy serializację do tablicy JSON
    // 16 -> "10", 0 -> "00", 3 -> "03" (padding do 1 bajtu, bo p=17 mieści się w 8 bitach)
    assert_eq!(encode_r(&SchnorrR::Field(val_ext)), "[\"10\",\"00\",\"03\"]");

    // 4. Encode((3, 5) \in E(F_17)) = {"x":"03","y":"05"}
    let p3 = BigInt::from(17);
    let f3 = FiniteField::new_prime_field(p3);
    
    // Dobieramy a i b tak, żeby punkt (3, 5) leżał na krzywej.
    // y^2 = x^3 + ax + b mod 17
    // 5^2 = 3^3 + a*3 + b
    // 25 = 27 + 3a + b => 8 = 10 + 3a + b mod 17
    // Niech a = 0, wtedy b = -2 = 15.
    let a_val = FieldElement::new(vec![BigInt::from(0)], f3.clone());
    let b_val = FieldElement::new(vec![BigInt::from(15)], f3.clone());

    let curve = EllipticCurve::new(a_val, b_val, f3.clone());

    let x = FieldElement::new(vec![BigInt::from(3)], f3.clone());
    let y = FieldElement::new(vec![BigInt::from(5)], f3.clone());
    
    let point = curve.point(x, y).expect("Punkt (3,5) powinien leżeć na krzywej y^2 = x^3 + 15");

    assert_eq!(encode_r(&SchnorrR::Curve(point)), r#"{"x":"03","y":"05"}"#);
}

// --- Testy Podpisu ---

#[test]
fn test_schnorr_field_sign_verify() {
    println!("--- Test: Schnorr Signature (Finite Field) ---");
    let p = BigInt::from(23);
    let f = FiniteField::new_prime_field(p);
    let g = FieldElement::new(vec![BigInt::from(5)], f);
    let order = BigInt::from(22);

    let schnorr = SchnorrField { g: g.clone(), order: order.clone() };

    let sk = BigInt::from(4);
    let pk = g.pow(&sk);
    let msg = "Hello Schnorr";
    
    let sig = schnorr.sign(msg, &sk);
    let valid = schnorr.verify(msg, &pk, &sig);
    assert!(valid, "Podpis powinien być poprawny");
}

#[test]
fn test_schnorr_curve_sign_verify() {
    println!("--- Test: Schnorr Signature (Elliptic Curve secp256k1) ---");
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::parse_bytes(p_hex.as_bytes(), 16).unwrap();
    let field = FiniteField::new_prime_field(p);

    let a = FieldElement::new(vec![BigInt::from(0)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(7)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    let gx_hex = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
    let gy_hex = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
    let gx = FieldElement::from_hex_str(gx_hex, field.clone());
    let gy = FieldElement::from_hex_str(gy_hex, field.clone());
    let g = curve.point(gx, gy).unwrap();

    let n_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
    let n = BigInt::parse_bytes(n_hex.as_bytes(), 16).unwrap();

    let schnorr = SchnorrCurve { g: g.clone(), order: n };

    let sk = BigInt::from(1234567890);
    let pk = g * &sk;
    let msg = "Blockchain Transaction";

    let sig = schnorr.sign(msg, &sk);
    let valid = schnorr.verify(msg, &pk, &sig);
    assert!(valid, "Podpis EC powinien być poprawny");
}