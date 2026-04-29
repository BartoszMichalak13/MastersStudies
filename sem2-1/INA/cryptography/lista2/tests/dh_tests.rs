use num_bigint::{BigInt, RandBigInt};
use num_traits::{One, Num};
use lista2::{FiniteField, FieldElement, EllipticCurve, ECPoint};

// --- Helper: Generowanie losowego klucza prywatnego ---
fn generate_private_key(order: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    // Klucz prywatny powinien być z przedziału [1, q-1]
    rng.gen_bigint_range(&BigInt::one(), order)
}

// --- Scenariusz A, B, C: Diffie-Hellman na Ciałach Skończonych ---
fn run_dh_field_scenario(name: &str, g: FieldElement, order: BigInt) {
    println!("--- DH Protocol: {} ---", name);

    // 1. Alicja generuje klucz
    let alice_sk = generate_private_key(&order);
    let alice_pk = g.pow(&alice_sk); // pk = g^sk

    // 2. Bob generuje klucz
    let bob_sk = generate_private_key(&order);
    let bob_pk = g.pow(&bob_sk);     // pk = g^sk

    // 3. Wymiana i obliczenie wspólnego sekretu
    // Alicja: (Bob_PK)^Alice_SK
    let alice_secret = bob_pk.pow(&alice_sk);
    
    // Bob: (Alice_PK)^Bob_SK
    let bob_secret = alice_pk.pow(&bob_sk);

    println!("Alice SK: {}", alice_sk);
    println!("Bob SK:   {}", bob_sk);
    println!("Shared Secret: {}", alice_secret);

    // Weryfikacja
    assert_eq!(alice_secret.coeffs, bob_secret.coeffs, "Shared secrets do not match!");
    println!(">> SUKCES: Sekrety są identyczne.\n");
}

// --- Scenariusz D: Diffie-Hellman na Krzywych Eliptycznych ---
fn run_dh_curve_scenario(name: &str, g: ECPoint, order: BigInt) {
    println!("--- DH Protocol (EC): {} ---", name);

    // 1. Alicja generuje klucz
    let alice_sk = generate_private_key(&order);
    // Na krzywych potęgowanie to mnożenie skalarne: pk = sk * G
    let alice_pk = g.clone() * &alice_sk; 

    // 2. Bob generuje klucz
    let bob_sk = generate_private_key(&order);
    let bob_pk = g.clone() * &bob_sk;

    // 3. Wymiana
    // Alicja: Alice_SK * Bob_PK
    let alice_secret = bob_pk * &alice_sk;
    
    // Bob: Bob_SK * Alice_PK
    let bob_secret = alice_pk * &bob_sk;

    // Weryfikacja (ECPoint ma zaimplementowane PartialEq)
    match (alice_secret, bob_secret) {
        (ECPoint::Point { x: x1, y: y1, .. }, ECPoint::Point { x: x2, y: y2, .. }) => {
            println!("Shared Secret X: {}", x1);
            assert_eq!(x1.coeffs, x2.coeffs, "Współrzędne X sekretów są różne!");
            assert_eq!(y1.coeffs, y2.coeffs, "Współrzędne Y sekretów są różne!");
            println!(">> SUKCES: Sekrety są identyczne.\n");
        },
        _ => panic!("Sekret wyszedł Infinity, co jest mało prawdopodobne (chyba że sk wylosowało się jako wielokrotność rzędu)"),
    }
}

// ================= TESTY KONKRETNYCH PRZYPADKÓW =================

#[test]
fn test_dh_case_a_classic_fp() {
    // (a) Classic F_p
    // Przykład: Grupa multiplikatywna modulo p=23. Rząd q=22.
    // Generator g=5 (5 jest generatorem w F_23)
    let p = BigInt::from(23);
    let field = FiniteField::new_prime_field(p);
    
    let g = FieldElement::new(vec![BigInt::from(5)], field.clone());
    let q = BigInt::from(22); // Rząd grupy generowanej przez 5

    run_dh_field_scenario("Classic F_p (p=23)", g, q);
}

#[test]
fn test_dh_case_b_characteristic_2() {
    // (b) Characteristic 2 field F_2^k
    // Ciało F_2^4 z wielomianem x^4 + x + 1.
    // Generator g = x (wielomian [0, 1]).
    // Rząd grupy multiplikatywnej to 2^4 - 1 = 15.
    let p = BigInt::from(2);
    let modulus = vec![1, 1, 0, 0, 1].into_iter().map(BigInt::from).collect();
    let field = FiniteField::new_extension_field(p, modulus);

    let g = FieldElement::from_bit_representation(BigInt::from(2), field.clone()); // 2 to '10' binarnie czyli 'x'
    let q = BigInt::from(15); 

    run_dh_field_scenario("Binary Field F_2^4", g, q);
}

#[test]
fn test_dh_case_c_extended_prime_field() {
    // (c) Extended prime field F_p^k
    // Ciało F_3^2 z modułem x^2 + 1.
    // Elementów jest 3^2 = 9. Rząd grupy multiplikatywnej to 8.
    // Generator g = x + 1 ([1, 1]).
    let p = BigInt::from(3);
    let modulus = vec![BigInt::from(1), BigInt::from(0), BigInt::from(1)];
    let field = FiniteField::new_extension_field(p, modulus);

    let g = FieldElement::new(vec![BigInt::from(1), BigInt::from(1)], field.clone());
    let q = BigInt::from(8);

    run_dh_field_scenario("Extended Field F_3^2", g, q);
}

#[test]
fn test_dh_case_d_elliptic_curve() {
    // (d) Elliptic Curve over F_p (secp256k1)
    
    // Parametry secp256k1
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::from_str_radix(p_hex, 16).unwrap();
    let field = FiniteField::new_prime_field(p);

    let a = FieldElement::new(vec![BigInt::from(0)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(7)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    // Generator G
    let gx_hex = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
    let gy_hex = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
    let gx = FieldElement::from_hex_str(gx_hex, field.clone());
    let gy = FieldElement::from_hex_str(gy_hex, field.clone());
    
    let g_point = curve.point(gx, gy).unwrap();

    // Rząd n (order of G)
    let n_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
    let n = BigInt::from_str_radix(n_hex, 16).unwrap();

    run_dh_curve_scenario("Elliptic Curve secp256k1", g_point, n);
}

#[test]
fn test_dh_case_d_binary_curve() {
    // (d) Elliptic Curve over F_2^m
    // F_2^4, y^2 + xy = x^3 + x^2 + 1
    // Punkt P = (1, 6) ma rząd 4.
    
    let p = BigInt::from(2);
    let modulus = vec![1, 1, 0, 0, 1].into_iter().map(BigInt::from).collect();
    let field = FiniteField::new_extension_field(p, modulus);

    let one = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    let curve = EllipticCurve::new(one.clone(), one.clone(), field.clone()); // a=1, b=1

    // Punkt P = (1, 6)
    let px = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    let py = FieldElement::from_bit_representation(BigInt::from(6), field.clone());
    let g_point = curve.point(px, py).unwrap();

    let order = BigInt::from(4);

    run_dh_curve_scenario("Binary Elliptic Curve (Small)", g_point, order);
}