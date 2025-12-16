use num_bigint::BigInt;
use std::rc::Rc;
use lista2::{FiniteField, FieldElement, EllipticCurve, ECPoint};

// --- Helper: Ciało F_17 ---
fn get_prime_field_17() -> Rc<FiniteField> {
    FiniteField::new_prime_field(BigInt::from(17))
}

// --- Helper: Ciało F_2^4 ---
fn get_binary_field_16() -> Rc<FiniteField> {
    let p = BigInt::from(2);
    // x^4 + x + 1
    let modulus = vec![1, 1, 0, 0, 1].into_iter().map(BigInt::from).collect();
    FiniteField::new_extension_field(p, modulus)
}

#[test]
fn test_ec_addition_prime_field() {
    println!("--- Test: Addition P + Q in F_17 ---");
    let field = get_prime_field_17();
    
    // Krzywa: y^2 = x^3 + 2x + 2 nad F_17
    let a = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    // Punkt P = (5, 1)
    let p_x = FieldElement::new(vec![BigInt::from(5)], field.clone());
    let p_y = FieldElement::new(vec![BigInt::from(1)], field.clone());
    let p = curve.point(p_x, p_y).expect("P invalid");

    // 1. Test PODWAJANIA (Doubling): P + P = 2P
    // Ręczne wyliczenie:
    // lambda = (3*5^2 + 2) / (2*1) = 77 / 2 = 9 / 2 = 9 * 9 = 81 = 13 (mod 17)
    // x3 = 13^2 - 5 - 5 = 169 - 10 = 159 = 6 (mod 17)
    // y3 = 13(5 - 6) - 1 = -13 - 1 = -14 = 3 (mod 17)
    // Oczekujemy 2P = (6, 3)
    
    let p2 = p.clone() + p.clone(); // Używa operatora Add
    
    match p2.clone() {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.coeffs[0], BigInt::from(6), "Błąd x w 2P");
            assert_eq!(y.coeffs[0], BigInt::from(3), "Błąd y w 2P");
        },
        _ => panic!("2P powinno być punktem"),
    }

    // 2. Test DODAWANIA RÓŻNYCH PUNKTÓW: P + 2P = 3P
    // P = (5, 1), 2P = (6, 3)
    // lambda = (3 - 1) / (6 - 5) = 2 / 1 = 2
    // x3 = 2^2 - 5 - 6 = 4 - 11 = -7 = 10 (mod 17)
    // y3 = 2(5 - 10) - 1 = 2(-5) - 1 = -11 = 6 (mod 17)
    // Oczekujemy 3P = (10, 6)

    let p3 = p.clone() + p2.clone(); // P + 2P

    match p3 {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.coeffs[0], BigInt::from(10), "Błąd x w 3P (P + 2P)");
            assert_eq!(y.coeffs[0], BigInt::from(6), "Błąd y w 3P (P + 2P)");
        },
        _ => panic!("3P powinno być punktem"),
    }
}

#[test]
fn test_ec_multiplication_prime_field() {
    println!("--- Test: Scalar Multiplication n * P in F_17 ---");
    let field = get_prime_field_17();
    let a = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let b = FieldElement::new(vec![BigInt::from(2)], field.clone());
    let curve = EllipticCurve::new(a, b, field.clone());

    let p = curve.point(
        FieldElement::new(vec![BigInt::from(5)], field.clone()),
        FieldElement::new(vec![BigInt::from(1)], field.clone())
    ).unwrap();

    // Test: 3 * P (używając Mul<&BigInt>)
    // Powinno dać to samo co ręczne dodawanie wyżej: (10, 6)
    let p3_mul = p.clone() * &BigInt::from(3);

    match p3_mul {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.coeffs[0], BigInt::from(10), "Błąd w mnożeniu 3*P");
            assert_eq!(y.coeffs[0], BigInt::from(6), "Błąd w mnożeniu 3*P");
        },
        _ => panic!("3*P powinno być punktem"),
    }

    // Test łączności: 2*P + P == 3*P
    let p2 = p.clone() * &BigInt::from(2);
    let sum = p2 + p.clone();
    let mul = p.clone() * &BigInt::from(3);
    
    assert!(sum == mul, "Mnożenie skalarne niezgodne z dodawaniem");
}

#[test]
fn test_ec_arithmetic_binary_field() {
    println!("--- Test: Arithmetic in F_2^4 ---");
    let field = get_binary_field_16();
    // Krzywa: y^2 + xy = x^3 + x^2 + 1 (a=1, b=1)
    let one = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    let curve = EllipticCurve::new(one.clone(), one.clone(), field.clone());

    // P = (1, 6)
    let p = curve.point(
        FieldElement::from_bit_representation(BigInt::from(1), field.clone()),
        FieldElement::from_bit_representation(BigInt::from(6), field.clone())
    ).expect("P invalid");

    // 1. Sprawdźmy czy 2*P (mnożenie) daje to samo co P + P (dodawanie)
    let p2_add = p.clone() + p.clone();
    let p2_mul = p.clone() * &BigInt::from(2);
    
    match &p2_add {
        ECPoint::Infinity { .. } => panic!("2P nie powinno być Infinity dla tego punktu"),
        _ => {}
    }
    
    assert!(p2_add == p2_mul, "P+P != 2*P w ciele binarnym");

    // 2. Sprawdźmy czy 3*P = P + 2P
    let p3_mul = p.clone() * &BigInt::from(3);
    let p3_add = p.clone() + p2_add.clone();

    assert!(p3_add == p3_mul, "P+2P != 3*P w ciele binarnym");
    
    // 3. Test dużego skalara (np. 15)
    // 15 = 1111 binarnie. Testuje algorytm Double-and-Add
    let p15 = p.clone() * &BigInt::from(15);
    
    // Alternatywnie: 15P = 16P - P = 2(8P) - P
    let p16 = (p.clone() * &BigInt::from(8)) * &BigInt::from(2);
    let p15_check = p16 + p.neg();
    
    assert!(p15 == p15_check, "15P != 16P - P");
}