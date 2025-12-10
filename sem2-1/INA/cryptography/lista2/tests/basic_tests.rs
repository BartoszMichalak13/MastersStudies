use num_bigint::BigInt;
use std::ops::Neg;
use lista2::{FiniteField, FieldElement}; 

#[test]
fn test_prime_field_f11() {
    // --- Konfiguracja ---
    let p = BigInt::from(11);
    let f11 = FiniteField::new_prime_field(p);

    let a = FieldElement::new(vec![BigInt::from(3)], f11.clone());
    let b = FieldElement::new(vec![BigInt::from(9)], f11.clone());

    // 1. Dodawanie
    let sum = a.clone() + b.clone();
    assert_eq!(sum.coeffs, vec![BigInt::from(1)], "Błąd w dodawaniu a + b");

    // 2. Mnożenie
    let prod = a.clone() * b.clone();
    assert_eq!(prod.coeffs, vec![BigInt::from(5)], "Błąd w mnożeniu a * b");

    // 3. Inwersja
    let inv_a = a.clone().inverse();
    assert_eq!(inv_a.coeffs, vec![BigInt::from(4)], "Błąd w inwersji a");

    // 4. Dzielenie
    let div = a.clone() / b.clone();
    assert_eq!(div.coeffs, vec![BigInt::from(4)], "Błąd w dzieleniu a / b");

    // 5. Tożsamość
    let identity = a.clone() * inv_a;
    assert_eq!(identity.coeffs, vec![BigInt::from(1)], "a * a^-1 nie dało 1");

    // 6. Potęgowanie
    let pow = a.pow(&BigInt::from(5));
    assert_eq!(pow.coeffs, vec![BigInt::from(1)], "Błąd w potęgowaniu a^5");

    // 7. Negacja
    let neg = a.clone().neg();
    assert_eq!(neg.coeffs, vec![BigInt::from(8)], "Błąd w negacji a");
}

#[test]
fn test_extension_field_f9() {
    let p3 = BigInt::from(3);
    let modulus = vec![BigInt::from(1), BigInt::from(0), BigInt::from(1)];
    let f9 = FiniteField::new_extension_field(p3, modulus);

    let g = FieldElement::new(vec![BigInt::from(1), BigInt::from(1)], f9.clone());
    let h = FieldElement::new(vec![BigInt::from(0), BigInt::from(2)], f9.clone());

    // 1. Dodawanie
    let sum = g.clone() + h.clone();
    assert_eq!(sum.coeffs, vec![BigInt::from(1)], "Błąd w dodawaniu g + h");

    // 2. Mnożenie
    let prod = g.clone() * h.clone();
    assert_eq!(prod.coeffs, vec![BigInt::from(1), BigInt::from(2)], "Błąd w mnożeniu g * h");

    // 3. Potęgowanie
    let pow = g.pow(&BigInt::from(8));
    assert_eq!(pow.coeffs, vec![BigInt::from(1)], "g^8 nie równa się 1");

    // 4. Inwersja
    let inv_g = g.clone().inverse();
    assert_eq!(inv_g.coeffs, vec![BigInt::from(2), BigInt::from(1)], "Błędna inwersja g");

    // 5. Negacja
    let neg_g = g.clone().neg();
    assert_eq!(neg_g.coeffs, vec![BigInt::from(2), BigInt::from(2)], "Błędna negacja g");

    // 6. Dzielenie
    let m = FieldElement::new(vec![BigInt::from(1), BigInt::from(2)], f9.clone());
    let n = FieldElement::new(vec![BigInt::from(0), BigInt::from(1)], f9.clone());
    let div = m.clone() / n.clone();
    assert_eq!(div.coeffs, vec![BigInt::from(2), BigInt::from(2)], "Błędne dzielenie m / n");
}