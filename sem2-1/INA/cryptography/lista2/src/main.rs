// use num_bigint::BigInt;
// use num_integer::Integer;
// use num_traits::{One, Zero};
// use std::ops::{Add, Div, Mul, Neg, Sub};
// use std::rc::Rc;
// use std::fmt;
// use num_traits::Num; // Potrzebne do parsowania hex

// // --- Helper Functions ---

// // Rozszerzony algorytm Euklidesa dla liczb całkowitych
// fn egcd_bigint(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
//     if a.is_zero() {
//         (b.clone(), BigInt::zero(), BigInt::one())
//     } else {
//         let (g, x, y) = egcd_bigint(&(b % a), a);
//         (g, y - (b / a) * &x, x)
//     }
// }

// // Inwersja modularna dla skalarów
// fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
//     let (g, x, _) = egcd_bigint(a, m);
//     if g != BigInt::one() {
//         None
//     } else {
//         Some((x % m + m) % m)
//     }
// }

// // --- Structures ---

// #[derive(Debug, PartialEq)]
// pub struct FiniteField {
//     pub p: BigInt,
//     pub k: usize,
//     pub modulus: Vec<BigInt>,
// }

// #[derive(Clone)]
// pub struct FieldElement {
//     pub coeffs: Vec<BigInt>,
//     pub field: Rc<FiniteField>,
// }

// impl FiniteField {
//     pub fn new_prime_field(p: BigInt) -> Rc<Self> {
//         Rc::new(FiniteField { p, k: 1, modulus: vec![] })
//     }

//     pub fn new_extension_field(p: BigInt, modulus: Vec<BigInt>) -> Rc<Self> {
//         let k = modulus.len() - 1;
//         Rc::new(FiniteField { p, k, modulus })
//     }
// }

// // --- Polynomial Arithmetic Logic ---

// impl FieldElement {
//     pub fn new(coeffs: Vec<BigInt>, field: Rc<FiniteField>) -> Self {
//         let mut element = FieldElement { coeffs, field };
//         element.normalize();
//         element
//     }

//     fn normalize(&mut self) {
//         let p = &self.field.p;
//         for c in &mut self.coeffs {
//             *c = c.mod_floor(p);
//         }
//         while self.coeffs.len() > 1 && self.coeffs.last().unwrap().is_zero() {
//             self.coeffs.pop();
//         }
//     }

//     fn poly_add(&self, other: &Self) -> Vec<BigInt> {
//         let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
//         let mut result = Vec::with_capacity(max_len);

//         let zero = BigInt::zero();

//         for i in 0..max_len {
//             let a = self.coeffs.get(i).unwrap_or(&zero);
//             let b = other.coeffs.get(i).unwrap_or(&zero);
//             result.push(a + b);
//         }
//         result
//     }

//     fn poly_mul_raw(&self, other: &Self) -> Vec<BigInt> {
//         if self.coeffs.is_empty() || other.coeffs.is_empty() {
//             return vec![BigInt::zero()];
//         }
//         let deg_a = self.coeffs.len() - 1;
//         let deg_b = other.coeffs.len() - 1;
//         let mut result = vec![BigInt::zero(); deg_a + deg_b + 1];

//         for (i, c1) in self.coeffs.iter().enumerate() {
//             for (j, c2) in other.coeffs.iter().enumerate() {
//                 result[i + j] += c1 * c2;
//             }
//         }
//         result
//     }

//     fn poly_div_rem(numerator: &[BigInt], denominator: &[BigInt], p: &BigInt) -> (Vec<BigInt>, Vec<BigInt>) {
//         let mut rem = numerator.to_vec();
//         let den_deg = denominator.len() - 1;
//         let den_lead = denominator.last().unwrap();

//         let den_lead_inv = mod_inverse(den_lead, p).expect("Leading coefficient not invertible in F_p");

//         let mut quo = vec![BigInt::zero(); if rem.len() > den_deg { rem.len() - den_deg } else { 1 }];

//         while rem.len() >= denominator.len() {
//             let rem_deg = rem.len() - 1;
//             let diff_deg = rem_deg - den_deg;

//             let rem_lead = rem.last().unwrap().clone();
//             let factor = (&rem_lead * &den_lead_inv).mod_floor(p);

//             if diff_deg < quo.len() {
//                quo[diff_deg] = factor.clone();
//             }

//             for i in 0..=den_deg {
//                 let val = &factor * &denominator[i];
//                 let target_idx = i + diff_deg;
//                 rem[target_idx] = (&rem[target_idx] - val).mod_floor(p);
//             }

//             while rem.len() > 0 && rem.last().unwrap().is_zero() {
//                 rem.pop();
//             }
//         }

//         if rem.is_empty() { rem.push(BigInt::zero()); }
//         (quo, rem)
//     }
// }

// // --- Operator Overloading ---

// impl Add for FieldElement {
//     type Output = Self;
//     fn add(self, other: Self) -> Self {
//         let coeffs = self.poly_add(&other);
//         FieldElement::new(coeffs, self.field.clone())
//     }
// }

// impl Sub for FieldElement {
//     type Output = Self;
//     fn sub(self, other: Self) -> Self {
//         self + (-other)
//     }
// }

// impl Neg for FieldElement {
//     type Output = Self;
//     fn neg(self) -> Self {
//         let mut new_coeffs = Vec::new();
//         for c in &self.coeffs {
//             new_coeffs.push(-c);
//         }
//         FieldElement::new(new_coeffs, self.field.clone())
//     }
// }

// impl Mul for FieldElement {
//     type Output = Self;
//     fn mul(self, other: Self) -> Self {
//         let raw_prod = self.poly_mul_raw(&other);

//         if self.field.k == 1 {
//              FieldElement::new(raw_prod, self.field.clone())
//         } else {
//             let (_, rem) = FieldElement::poly_div_rem(&raw_prod, &self.field.modulus, &self.field.p);
//             FieldElement::new(rem, self.field.clone())
//         }
//     }
// }

// impl FieldElement {
//     fn extended_euclidean_poly(a: Vec<BigInt>, b: Vec<BigInt>, p: &BigInt) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
//         let mut r0 = a;
//         let mut r1 = b;
//         let mut t0 = vec![BigInt::one()];
//         let mut t1 = vec![BigInt::zero()];
//         let s0 = vec![BigInt::zero()];

//         while !(r1.len() == 1 && r1[0].is_zero()) && !r1.is_empty() {
//              let (quo, rem) = FieldElement::poly_div_rem(&r0, &r1, p);

//              r0 = r1;
//              r1 = rem;

//              let quo_times_t1 = {
//                  let mut res = vec![BigInt::zero(); quo.len() + t1.len()];
//                  if !quo.is_empty() && !t1.is_empty() {
//                     res = vec![BigInt::zero(); quo.len() + t1.len() - 1];
//                      for (i, c1) in quo.iter().enumerate() {
//                         for (j, c2) in t1.iter().enumerate() {
//                             res[i+j] = (&res[i+j] + c1 * c2) % p;
//                         }
//                      }
//                  }
//                  while res.len() > 1 && res.last().unwrap().is_zero() { res.pop(); }
//                  res
//              };

//              let t_next = {
//                  let max_len = std::cmp::max(t0.len(), quo_times_t1.len());
//                  let mut res = Vec::with_capacity(max_len);

//                  let zero = BigInt::zero();

//                  for i in 0..max_len {
//                      let v1 = t0.get(i).unwrap_or(&zero);
//                      let v2 = quo_times_t1.get(i).unwrap_or(&zero);
//                      res.push((v1 - v2).mod_floor(p));
//                  }
//                  while res.len() > 1 && res.last().unwrap().is_zero() { res.pop(); }
//                  res
//              };

//              t0 = t1;
//              t1 = t_next;
//         }

//         (r0, t0, s0)
//     }

//     pub fn inverse(self) -> Self {
//         if self.coeffs.len() == 1 && self.coeffs[0].is_zero() {
//             panic!("Cannot invert zero");
//         }

//         if self.field.k == 1 {
//             let inv = mod_inverse(&self.coeffs[0], &self.field.p).expect("Inverse failed");
//             return FieldElement::new(vec![inv], self.field.clone());
//         }

//         let (gcd, mut t, _) = FieldElement::extended_euclidean_poly(
//             self.coeffs.clone(),
//             self.field.modulus.clone(),
//             &self.field.p
//         );

//         if gcd.len() > 1 {
//             panic!("GCD degree > 0, modulus likely not irreducible or element is zero divisor");
//         }

//         let scaler = mod_inverse(&gcd[0], &self.field.p).unwrap();

//         for c in &mut t {
//             *c = (c as &BigInt * &scaler).mod_floor(&self.field.p);
//         }

//         FieldElement::new(t, self.field.clone())
//     }
// }

// impl Div for FieldElement {
//     type Output = Self;
//     fn div(self, rhs: Self) -> Self {
//         self * rhs.inverse()
//     }
// }

// // --- Exponentiation ---

// impl FieldElement {
//     pub fn pow(&self, exp: &BigInt) -> Self {
//         let mut res = FieldElement::new(vec![BigInt::one()], self.field.clone());
//         let mut base = self.clone();
//         let mut e = exp.clone();

//         if e < BigInt::zero() {
//             base = base.inverse();
//             e = -e;
//         }

//         while e > BigInt::zero() {
//             if &e % 2 == BigInt::one() {
//                 res = res * base.clone();
//             }
//             base = base.clone() * base;
//             e /= 2;
//         }
//         res
//     }
// }

// impl FieldElement {
//     // --- Helper: Tworzenie z HEX stringa (dla dużych liczb) ---
//     pub fn from_hex_str(hex: &str, field: Rc<FiniteField>) -> Self {
//         let val = BigInt::from_str_radix(hex, 16).expect("Invalid Hex String");
//         // Jeśli k=1, to po prostu liczba
//         if field.k == 1 {
//              FieldElement::new(vec![val], field)
//         } else {
//              // Jeśli k>1 i podajemy jedną liczbę, zakładamy że to reprezentacja bitowa wielomianu (dla F_2^m)
//              FieldElement::from_bit_representation(val, field)
//         }
//     }

//     // --- F_2^m (Binary Fields) ---

//     // Konwertuje dużą liczbę całkowitą na wielomian nad F_2
//     // Np. liczba 5 (binarnie 101) -> wielomian 1*x^2 + 0*x + 1 -> coeffs: [1, 0, 1]
//     pub fn from_bit_representation(val: BigInt, field: Rc<FiniteField>) -> Self {
//         if field.p != BigInt::from(2) {
//              panic!("Bit representation is valid only for characteristic p=2");
//         }

//         let mut coeffs = Vec::new();
//         let mut temp = val;

//         // Wyciągamy bity od najmniej znaczącego (Little Endian dla wielomianów)
//         while temp > BigInt::zero() {
//             if &temp % 2 == BigInt::one() {
//                 coeffs.push(BigInt::one());
//             } else {
//                 coeffs.push(BigInt::zero());
//             }
//             temp /= 2;
//         }

//         // Uzupełnij zerami, jeśli wyszło puste (dla zera)
//         if coeffs.is_empty() { coeffs.push(BigInt::zero()); }

//         FieldElement::new(coeffs, field)
//     }

//     // Konwertuje wielomian nad F_2 z powrotem na liczbę (Bit String / Integer)
//     pub fn to_bit_representation(&self) -> BigInt {
//          if self.field.p != BigInt::from(2) {
//              panic!("Bit representation is valid only for characteristic p=2");
//         }

//         let mut res = BigInt::zero();
//         // coeffs[0] to x^0 (najmniej znaczący bit), coeffs[N] to x^N (najbardziej znaczący)
//         for (i, c) in self.coeffs.iter().enumerate().rev() {
//             res *= 2;
//             if c.is_one() {
//                 res += 1;
//             }
//         }
//         res
//     }
// }

// // --- Formatting ---

// impl fmt::Display for FieldElement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if self.coeffs.is_empty() || (self.coeffs.len() == 1 && self.coeffs[0].is_zero()) {
//             return write!(f, "0");
//         }

//         let mut first = true;
//         for (i, c) in self.coeffs.iter().enumerate().rev() {
//             if c.is_zero() { continue; }

//             if !first { write!(f, " + ")?; }

//             if i == 0 {
//                 write!(f, "{}", c)?;
//             } else if i == 1 {
//                  if c.is_one() { write!(f, "x")?; } else { write!(f, "{}x", c)?; }
//             } else {
//                  if c.is_one() { write!(f, "x^{}", i)?; } else { write!(f, "{}x^{}", c, i)?; }
//             }
//             first = false;
//         }
//         Ok(())
//     }
// }

// --- Main / Test ---
fn main() {
    println!("Main");
}


// #[cfg(test)]
// mod tests {
//     use super::*; // Importujemy wszystko z głównego modułu (struktury, funkcje)

//     #[test]
//     fn test_prime_field_f11() {
//         // --- Konfiguracja ---
//         let p = BigInt::from(11);
//         let f11 = FiniteField::new_prime_field(p);

//         // a = 3, b = 9
//         let a = FieldElement::new(vec![BigInt::from(3)], f11.clone());
//         let b = FieldElement::new(vec![BigInt::from(9)], f11.clone());

//         // --- Weryfikacja (Assertions) ---

//         // 1. Dodawanie: 3 + 9 = 12 = 1 (mod 11)
//         let sum = a.clone() + b.clone();
//         assert_eq!(sum.coeffs, vec![BigInt::from(1)], "Błąd w dodawaniu a + b");

//         // 2. Mnożenie: 3 * 9 = 27 = 5 (mod 11)
//         let prod = a.clone() * b.clone();
//         assert_eq!(prod.coeffs, vec![BigInt::from(5)], "Błąd w mnożeniu a * b");

//         // 3. Inwersja: 3^(-1) (mod 11) = 4
//         // Sprawdzenie: 3 * 4 = 12 = 1
//         let inv_a = a.clone().inverse();
//         assert_eq!(inv_a.coeffs, vec![BigInt::from(4)], "Błąd w inwersji a");

//         // 4. Dzielenie: 3 / 9 = 3 * 9^(-1) = 3 * 5 = 15 = 4 (mod 11)
//         let div = a.clone() / b.clone();
//         assert_eq!(div.coeffs, vec![BigInt::from(4)], "Błąd w dzieleniu a / b");

//         // 5. Sprawdzenie inwersji: a * a^(-1) musi dać 1
//         let identity = a.clone() * inv_a; // inv_a już sklonowane/stworzone wyżej
//         assert_eq!(identity.coeffs, vec![BigInt::from(1)], "a * a^-1 nie dało 1");

//         // 6. Potęgowanie: 3^5 = 243 = 1 (mod 11)
//         let pow = a.pow(&BigInt::from(5));
//         assert_eq!(pow.coeffs, vec![BigInt::from(1)], "Błąd w potęgowaniu a^5");

//         // 7. Negacja: -3 = 8 (mod 11)
//         let neg = a.clone().neg();
//         assert_eq!(neg.coeffs, vec![BigInt::from(8)], "Błąd w negacji a");

//     }

//     #[test]
//     fn test_extension_field_f9() {
//         // --- Konfiguracja F_{3^2} z modułem x^2 + 1 ---
//         let p3 = BigInt::from(3);
//         // Modulus: x^2 + 0x + 1 -> coeffs: [1, 0, 1]
//         let modulus = vec![BigInt::from(1), BigInt::from(0), BigInt::from(1)];
//         let f9 = FiniteField::new_extension_field(p3, modulus);

//         // g = x + 1 -> coeffs: [1, 1]
//         let g = FieldElement::new(vec![BigInt::from(1), BigInt::from(1)], f9.clone());
//         // h = 2x    -> coeffs: [0, 2]
//         let h = FieldElement::new(vec![BigInt::from(0), BigInt::from(2)], f9.clone());

//         // --- Weryfikacja ---

//         // 1. Dodawanie: (x + 1) + 2x = 3x + 1 = 0x + 1 = 1 (mod 3)
//         let sum = g.clone() + h.clone();
//         // Oczekujemy wektora [1] (znormalizowanego)
//         assert_eq!(sum.coeffs, vec![BigInt::from(1)], "Błąd w dodawaniu g + h");

//         // 2. Mnożenie: (x + 1) * 2x = 2x^2 + 2x
//         // Modulo x^2 + 1 (gdzie x^2 = -1 = 2):
//         // 2(2) + 2x = 4 + 2x = 1 + 2x
//         // Coeffs: [1, 2]
//         let prod = g.clone() * h.clone();
//         assert_eq!(
//             prod.coeffs,
//             vec![BigInt::from(1), BigInt::from(2)],
//             "Błąd w mnożeniu g * h"
//         );

//         // 3. Potęgowanie: g^8 powinno być 1 (rząd grupy multiplikatywnej to 3^2 - 1 = 8)
//         let pow = g.pow(&BigInt::from(8));
//         assert_eq!(pow.coeffs, vec![BigInt::from(1)], "g^8 nie równa się 1");

//         // 4. Inwersja g: g^(-1)
//         // Z obliczeń ręcznych wiemy, że to x + 2 (coeffs: [2, 1])
//         let inv_g = g.clone().inverse();
//         assert_eq!(
//             inv_g.coeffs,
//             vec![BigInt::from(2), BigInt::from(1)],
//             "Błędna inwersja g"
//         );

//         // 5. Sprawdzenie inwersji: g * g^(-1) == 1
//         let identity = g.clone() * inv_g;
//         assert_eq!(identity.coeffs, vec![BigInt::from(1)], "g * g^-1 nie dało 1");

//         // 6. Negacja g: -(x + 1) = -x - 1 = 2x + 2 (mod 3)
//         let neg_g = g.clone().neg();
//         assert_eq!(
//             neg_g.coeffs,
//             vec![BigInt::from(2), BigInt::from(2)],
//             "Błędna negacja g"
//         );

//         // m = 2x + 1 -> coeffs: [1, 2]
//         // n = x   -> coeffs: [0, 1]
//         let m = FieldElement::new(vec![BigInt::from(1), BigInt::from(2)], f9.clone());
//         let n = FieldElement::new(vec![BigInt::from(0), BigInt::from(1)], f9.clone());

//         // 7. Dzielenie: m / n = m * n^(-1)
//         let div = m.clone() / n.clone();
//         // Ręczne obliczenia dają wynik 1
//         assert_eq!(
//             div.coeffs,
//             vec![BigInt::from(2), BigInt::from(2)],
//             "Błędne dzielenie m / n"
//         );
//     }
// }


// #[cfg(test)]
// mod crypto_tests {
//     use super::*;
//     use num_traits::Num;

//     #[test]
//     fn test_secp256k1_prime_field() {
//         println!("--- Crypto Test: F_p (256-bit secp256k1) ---");

//         // P = 2^256 - 2^32 - 977 (Standard Bitcoin Prime)
//         let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
//         let p = BigInt::from_str_radix(p_hex, 16).unwrap();

//         let field = FiniteField::new_prime_field(p.clone());

//         // Dwie duże liczby losowe (tutaj wpisane na sztywno dla powtarzalności)
//         let a_hex = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
//         let b_hex = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

//         let a = FieldElement::from_hex_str(a_hex, field.clone());
//         let b = FieldElement::from_hex_str(b_hex, field.clone());

//         // 1. Test inwersji: a * a^(-1) == 1
//         let inv_a = a.clone().inverse();
//         let unity = a.clone() * inv_a;

//         assert_eq!(unity.coeffs[0], BigInt::one(), "Inwersja 256-bitowa nieudana!");

//         // 2. Test Małego Twierdzenia Fermata: a^(p-1) == 1
//         let p_minus_1 = &p - BigInt::one();
//         let fermat = a.pow(&p_minus_1);
//         assert_eq!(fermat.coeffs[0], BigInt::one(), "Fermat failed for 256-bit!");
//     }

//     #[test]
//     fn test_binary_field_aes() {
//         println!("--- Crypto Test: F_{{2^8}} (AES) ---");

//         // AES używa ciała F_{2^8} z wielomianem nierozkładalnym:
//         // x^8 + x^4 + x^3 + x + 1 (Hex: 11B, Bin: 100011011)
//         // P = 2
//         let p = BigInt::from(2);

//         // Modulus coefficients (Little Endian): 1 + 1x + 0x^2 + 1x^3 + 1x^4 + 0... + 1x^8
//         // 1, 1, 0, 1, 1, 0, 0, 0, 1
//         let modulus_coeffs = vec![1, 1, 0, 1, 1, 0, 0, 0, 1]
//             .into_iter().map(BigInt::from).collect();

//         let aes_field = FiniteField::new_extension_field(p, modulus_coeffs);

//         // W AES często podaje się elementy jako bajty (liczby 0-255).
//         // Np. mnożenie: 0x57 * 0x83 = 0xC1 (w notacji wielomianowej AES)
//         // 0x57 = 01010111 (x^6 + x^4 + x^2 + x + 1)
//         // 0x83 = 10000011 (x^7 + x + 1)

//         let a = FieldElement::from_bit_representation(BigInt::from(0x57), aes_field.clone());
//         let b = FieldElement::from_bit_representation(BigInt::from(0x83), aes_field.clone());

//         let result = a * b;
//         let result_int = result.to_bit_representation();

//         // Oczekiwany wynik w AES dla 0x57 * 0x83 to 0xC1 (193)
//         assert_eq!(result_int, BigInt::from(0xC1), "Błąd mnożenia w ciele AES F_2^8");
//     }
// }

// #[cfg(test)]
// mod extended_tests {
//     use super::*;
//     use num_traits::Num;

//     // Pomocnik do tworzenia ciała AES (F_2^8)
//     fn get_aes_field() -> Rc<FiniteField> {
//         let p = BigInt::from(2);
//         // Wielomian AES: x^8 + x^4 + x^3 + x + 1 (Hex: 11B)
//         // Coeffs: [1, 1, 0, 1, 1, 0, 0, 0, 1]
//         let modulus_coeffs = vec![1, 1, 0, 1, 1, 0, 0, 0, 1]
//             .into_iter().map(BigInt::from).collect();
//         FiniteField::new_extension_field(p, modulus_coeffs)
//     }

//     // Pomocnik do tworzenia ciała secp256k1 (F_p)
//     fn get_secp_field() -> Rc<FiniteField> {
//         let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
//         let p = BigInt::from_str_radix(p_hex, 16).unwrap();
//         FiniteField::new_prime_field(p)
//     }

//     #[test]
//     fn test_exponentiation_edge_cases() {
//         let field = get_secp_field();
//         let a = FieldElement::from_hex_str("DEADBEEF", field.clone());

//         // Test 1: a^0 = 1
//         let pow_zero = a.pow(&BigInt::zero());
//         assert_eq!(pow_zero.coeffs[0], BigInt::one(), "a^0 powinno być 1");

//         // Test 2: a^1 = a
//         let pow_one = a.pow(&BigInt::one());
//         assert_eq!(pow_one.coeffs, a.coeffs, "a^1 powinno być a");

//         // Test 3: a^2 = a * a
//         let pow_two = a.pow(&BigInt::from(2));
//         let mul_self = a.clone() * a.clone();
//         assert_eq!(pow_two.coeffs, mul_self.coeffs, "a^2 powinno być równe a * a");
//     }

//     #[test]
//     fn test_algebraic_properties_distributivity() {
//         // Test rozdzielności mnożenia względem dodawania: a * (b + c) = a*b + a*c
//         // To jest "Ultimate Test" poprawności struktury ciała.

//         let field = get_secp_field();
//         let a = FieldElement::from_hex_str("12345", field.clone());
//         let b = FieldElement::from_hex_str("ABCDE", field.clone());
//         let c = FieldElement::from_hex_str("67890", field.clone());

//         // Lewa strona: a * (b + c)
//         let b_plus_c = b.clone() + c.clone();
//         let lhs = a.clone() * b_plus_c;

//         // Prawa strona: a*b + a*c
//         let a_mul_b = a.clone() * b.clone();
//         let a_mul_c = a.clone() * c.clone();
//         let rhs = a_mul_b + a_mul_c;

//         assert_eq!(lhs.coeffs, rhs.coeffs, "Prawo rozdzielności nie działa!");
//     }

//     #[test]
//     fn test_division_large_numbers() {
//         // Sprawdzenie: (a / b) * b == a
//         let field = get_secp_field();

//         let a = FieldElement::from_hex_str("11223344556677889900", field.clone());
//         let b = FieldElement::from_hex_str("AABBCCDDEEFF", field.clone());

//         let div_result = a.clone() / b.clone();
//         let check = div_result * b.clone();

//         assert_eq!(check.coeffs, a.coeffs, "Dzielenie nie jest odwrotnością mnożenia!");
//     }

//     #[test]
//     fn test_aes_inversion_known_vector() {
//         // W standardzie AES istnieje tabela S-Box, która opiera się na inwersji w GF(2^8).
//         // Znany wektor testowy: inwersja 0x53 powinna dać 0xCA.
//         // 0x53 = 01010011 (x^6 + x^4 + x + 1)
//         // 0xCA = 11001010 (x^7 + x^6 + x^3 + x)

//         let field = get_aes_field();
//         let a = FieldElement::from_bit_representation(BigInt::from(0x53), field.clone());

//         let inv_a = a.inverse();
//         let inv_val = inv_a.to_bit_representation();

//         assert_eq!(inv_val, BigInt::from(0xCA), "Błędna inwersja w AES (oczekiwano 0xCA)");
//     }

//     #[test]
//     fn test_freshmans_dream_in_binary_field() {
//         // W ciałach o charakterystyce 2 (p=2) zachodzi "Marzenie Pierwszoroczniaka":
//         // (a + b)^2 = a^2 + b^2
//         // To nie działa w zwykłej matematyce, ale w kryptografii binarnej TAK.
//         // Jeśli to przejdzie, Twoja obsługa p=2 jest perfekcyjna.

//         let field = get_aes_field();
//         let a = FieldElement::from_bit_representation(BigInt::from(0x57), field.clone());
//         let b = FieldElement::from_bit_representation(BigInt::from(0x83), field.clone());

//         // Lewa: (a + b)^2
//         let sum = a.clone() + b.clone();
//         let lhs = sum.pow(&BigInt::from(2));

//         // Prawa: a^2 + b^2
//         let a2 = a.pow(&BigInt::from(2));
//         let b2 = b.pow(&BigInt::from(2));
//         let rhs = a2 + b2;

//         // Uwaga: w porównaniu wektorów musimy uważać na normalizację,
//         // ale Twoja struktura robi to automatycznie w FieldElement::new.
//         // Dla pewności konwertujemy do liczb.
//         assert_eq!(lhs.to_bit_representation(), rhs.to_bit_representation(),
//             "Freshman's Dream nie działa w ciele binarnym!");
//     }
// }