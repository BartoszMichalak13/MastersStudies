use num_bigint::BigInt;
use num_traits::Num;
use lista2::{FiniteField, FieldElement, EllipticCurve, ECPoint};

#[test]
fn test_ec_small_field() {
    // Krzywa: y^2 = x^3 + 2x + 2 nad F_17
    // Parametry: a = 2, b = 2, p = 17
    let p = BigInt::from(17);
    let field = FiniteField::new_prime_field(p);
    
    let a = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(2)], field.clone());
    
    let curve = EllipticCurve::new(a, b, field.clone());

    // Punkt G = (5, 1)
    // Sprawdźmy czy leży na krzywej:
    // L = 1^2 = 1
    // P = 5^3 + 2*5 + 2 = 125 + 10 + 2 = 137
    // 137 mod 17 = 8 * 17 + 1 = 1. Zgadza się!
    
    let g_x = FieldElement::new(vec![BigInt::from(5)], field.clone());
    let g_y = FieldElement::new(vec![BigInt::from(1)], field.clone());
    
    // Używamy unwrap, bo oczekujemy, że punkt jest poprawny
    let g = curve.point(g_x, g_y).expect("Punkt (5,1) powinien leżeć na krzywej");

    // 1. Podwajanie: 2G
    // lambda = (3*5^2 + 2) / (2*1) = (75 + 2) / 2 = 77 / 2 = 9 / 2
    // w F_17: 1/2 = 9 (bo 2*9 = 18 = 1).
    // lambda = 9 * 9 = 81 = 13 (mod 17)
    // x3 = 13^2 - 5 - 5 = 169 - 10 = 159 = 6 (mod 17)
    // y3 = 13(5 - 6) - 1 = 13(-1) - 1 = -13 - 1 = -14 = 3 (mod 17)
    // Wynik 2G = (6, 3)
    let g2 = g.clone() + g.clone();
    
    match g2 {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.coeffs, vec![BigInt::from(6)], "Błąd x w 2G");
            assert_eq!(y.coeffs, vec![BigInt::from(3)], "Błąd y w 2G");
        },
        _ => panic!("2G nie powinno być Infinity"),
    }

    // 2. Mnożenie skalarne
    // 3G = 2G + G = (6,3) + (5,1)
    // lambda = (1 - 3) / (5 - 6) = -2 / -1 = 2
    // x3 = 2^2 - 6 - 5 = 4 - 11 = -7 = 10
    // y3 = 2(6 - 10) - 3 = 2(-4) - 3 = -8 - 3 = -11 = 6
    // Wynik 3G = (10, 6)
    let g3 = g.clone() * &BigInt::from(3);
    match g3 {
         ECPoint::Point { x, y, .. } => {
            assert_eq!(x.coeffs, vec![BigInt::from(10)], "Błąd x w 3G");
            assert_eq!(y.coeffs, vec![BigInt::from(6)], "Błąd y w 3G");
        },
        _ => panic!("3G nie powinno być Infinity"),
    }
}

#[test]
fn test_ec_secp256k1_generator() {
    println!("--- EC Test: secp256k1 ---");
    
    // Parametry secp256k1
    // p = 2^256 - 2^32 - 977
    let p_str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
    let p = BigInt::from_str_radix(p_str, 16).unwrap();
    let field = FiniteField::new_prime_field(p);

    // Krzywa: y^2 = x^3 + 7 (a=0, b=7)
    let a = FieldElement::new(vec![BigInt::from(0)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(7)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    // Generator G (standardowy punkt bazowy)
    let gx_str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
    let gy_str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";
    
    let gx = FieldElement::from_hex_str(gx_str, field.clone());
    let gy = FieldElement::from_hex_str(gy_str, field.clone());
    
    let g = curve.point(gx, gy).expect("Generator secp256k1 jest nieprawidłowy!");

    // Test: Rząd grupy secp256k1 (oznaczany jako n).
    // Własność: n * G = Infinity (O)
    // n = FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141
    let n_str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
    let n = BigInt::from_str_radix(n_str, 16).unwrap();

    let result = g.clone() * &n;
    
    match result {
        ECPoint::Infinity { .. } => {
            // Sukces! n*G to punkt w nieskończoności
        },
        _ => panic!("n * G powinno dać Infinity dla secp256k1"),
    }
}

#[test]
fn test_ec_extension_field_odd_characteristic() {
    println!("--- Test: Curve over F_{{5^2}} (Case E(F_p^k)) ---");

    // 1. Ciało F_{5^2}: p=5, modulus = x^2 + 2 (jest nierozkładalny w F_5)
    // Sprawdzenie nierozkładalności x^2+2:
    // 0^2+2=2, 1^2+2=3, 2^2+2=1, 3^2+2=11=1, 4^2+2=18=3. Brak zer.
    let p = BigInt::from(5);
    let modulus = vec![BigInt::from(2), BigInt::from(0), BigInt::from(1)]; // 2 + 0x + 1x^2
    let field = FiniteField::new_extension_field(p, modulus);

    // 2. Krzywa Weierstrassa: y^2 = x^3 + x (a=1, b=0)
    // Uwaga: Dla p=5 można używać Weierstrassa (p > 3).
    let a = FieldElement::new(vec![BigInt::from(1)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(0)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    // 3. Punkt na krzywej
    // Niech x = 2 (stała).
    // RHS = x^3 + x = 8 + 2 = 10 = 0 mod 5.
    // LHS = y^2. Szukamy y^2 = 0 -> y = 0.
    // Punkt P = (2, 0).

    let px = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let py = FieldElement::new(vec![BigInt::from(0)], field.clone());

    let p_point = curve.point(px, py).expect("Punkt (2,0) powinien być na krzywej");

    // 4. Operacja: Podwajanie
    // Wzór Weierstrassa: lambda = (3x^2 + a) / 2y.
    // Tutaj y=0, więc mianownik to 0 -> Dzielenie przez zero.
    // Zgodnie z teorią, punkt z y=0 na krzywej Weierstrassa ma rząd 2.
    // 2 * P powinno dać Infinity.

    let p2 = p_point.clone() + p_point.clone();
    match p2 {
        ECPoint::Infinity { .. } => {}, // Oczekiwany wynik
        _ => panic!("Punkt (2,0) ma rząd 2, więc 2P musi być Infinity"),
    }
}