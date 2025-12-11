use num_bigint::BigInt;
use std::rc::Rc;
use lista2::{FiniteField, FieldElement, EllipticCurve, ECPoint};

// Helper: Ciało F_{2^4} z wielomianem x^4 + x + 1 (Hex: 13, Bin: 10011)
fn get_binary_field() -> Rc<FiniteField> {
    let p = BigInt::from(2);
    // coeffs: [1, 1, 0, 0, 1] -> 1 + x + x^4
    let modulus = vec![1, 1, 0, 0, 1].into_iter().map(BigInt::from).collect();
    FiniteField::new_extension_field(p, modulus)
}

#[test]
fn test_binary_curve_basic_arithmetic() {
    let field = get_binary_field();
    
    // Definiujemy krzywą: y^2 + xy = x^3 + ax^2 + b
    // Parametry przykładowe: a = 1, b = 1
    // (Pamiętajmy: w ciele binarnym 1 to wielomian stały [1])
    let one = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    
    let curve = EllipticCurve::new(one.clone(), one.clone(), field.clone()); // a=1, b=1

    // Znajdźmy punkt na krzywej.
    // Niech x = g (generator pola, czyli wielomian "x", binarnie "2" lub "0010")
    // Musimy znaleźć y, które spełnia równanie.
    // Dla tego testu użyjemy zweryfikowanego punktu P = (x^3, x^3 + 1)
    // W reprezentacji bitowej dla F_{2^4}:
    // x = x^3 -> bitowo 8 (1000)
    // y = x^3 + 1 -> bitowo 9 (1001)
    
    let px = FieldElement::from_bit_representation(BigInt::from(0), field.clone());
    let py = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    
    // 1. Walidacja: Czy punkt leży na krzywej?
    let p = curve.point(px.clone(), py.clone())
        .expect("Punkt (0, 1) powinien leżeć na krzywej y^2+xy=x^3+x^2+1 nad F_{2^4}");

    // 2. Inwersja w ciele binarnym
    // Wzór: -P = (x, x + y)
    // px = 0 (0000), py = 1 (0001)
    // x + y = 0000 XOR 0001 = 0001 = 1
    let neg_p = p.neg();
    match neg_p {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.to_bit_representation(), BigInt::from(0), "x inwersji się nie zgadza");
            assert_eq!(y.to_bit_representation(), BigInt::from(1), "y inwersji to x+y");
        },
        _ => panic!("Inverse should be a point"),
    }

    // 3. Dodawanie odwrotności: P + (-P) = Infinity
    let sum_inv = p.clone() + p.neg();
    match sum_inv {
        ECPoint::Infinity { .. } => {}, // OK
        _ => panic!("P + (-P) powinno dać Infinity"),
    }

    // 4. Podwajanie: 2P dla punktu (0,1)
    let double_p = p.clone() + p.clone();

    match double_p {
        ECPoint::Infinity { .. } => {}, // To jest OCZEKIWANY wynik dla x=0
        _ => panic!("2P dla punktu (0,1) powinno być Infinity!"),
    }
}

#[test]
fn test_binary_curve_point_1_6() {
    let field = get_binary_field();
    // Parametry krzywej: a=1, b=1
    let one = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    let curve = EllipticCurve::new(one.clone(), one.clone(), field.clone());

    // Punkt P = (1, 6)
    // x = 1  (bin: 0001)
    // y = 6  (bin: 0110, wielomian x^2 + x)
    let px = FieldElement::from_bit_representation(BigInt::from(1), field.clone());
    let py = FieldElement::from_bit_representation(BigInt::from(6), field.clone());

    // 1. Sprawdzenie czy punkt leży na krzywej
    // Równanie: y^2 + xy = x^3 + x^2 + 1
    // Dla (1, 6):
    // LHS: 6^2 + 1*6.
    //    6^2 = (x^2+x)^2 = x^4+x^2 = (x+1)+x^2 = 7 (0111).
    //    1*6 = 6 (0110).
    //    LHS = 7 + 6 = 0111 XOR 0110 = 0001 = 1.
    // RHS: 1^3 + 1*1^2 + 1 = 1 + 1 + 1 = 1.
    // 1 == 1 -> Punkt leży na krzywej.

    let p = curve.point(px.clone(), py.clone())
        .expect("Punkt (1, 6) powinien leżeć na krzywej");

    // 2. Inwersja -P
    // Wzór: -P = (x, x + y)
    // x = 1 (0001), y = 6 (0110)
    // x + y = 0111 (7)
    // Oczekujemy punktu (1, 7)
    let neg_p = p.neg();
    match neg_p {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.to_bit_representation(), BigInt::from(1), "x inwersji powinno być 1");
            assert_eq!(y.to_bit_representation(), BigInt::from(7), "y inwersji powinno być 7");
        },
        _ => panic!("Inverse should be a point"),
    }

    // 3. Dodawanie P + (-P)
    let sum_inf = p.clone() + p.neg();
    match sum_inf {
        ECPoint::Infinity { .. } => {}, // OK
        _ => panic!("P + (-P) musi być Infinity"),
    }

    // 4. Podwajanie 2P
    // Obliczmy to ręcznie:
    // lambda = x + y/x = 1 + 6/1 = 1 + 6 = 7.
    // x3 = lambda^2 + lambda + a = 7^2 + 7 + 1.
    //    7^2 = (x^2+x+1)^2 = x^4+x^2+1 = (x+1)+x^2+1 = x^2+x = 6.
    //    x3 = 6 + 7 + 1 = 0110 XOR 0111 XOR 0001 = 0.
    // y3 = x^2 + (lambda + 1)x3 = 1^2 + (7+1)*0 = 1 + 0 = 1.
    // Oczekiwany wynik: (0, 1)

    let double_p = p.clone() + p.clone();
    match double_p {
        ECPoint::Point { x, y, .. } => {
            assert_eq!(x.to_bit_representation(), BigInt::from(0), "x w 2P powinno być 0");
            assert_eq!(y.to_bit_representation(), BigInt::from(1), "y w 2P powinno być 1");
        },
        _ => panic!("2P nie powinno być Infinity"),
    }
}