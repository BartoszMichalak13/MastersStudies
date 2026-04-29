
use crate::finite_field::{FiniteField, FieldElement};
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::rc::Rc;

pub struct GHash {
    field: Rc<FiniteField>,
    h: FieldElement,
}

impl GHash {
    pub fn new(h_hex: &str) -> Self {
        // 1. Ciało GF(2^128)
        let p = BigInt::from(2);
        let mut modulus_coeffs = vec![BigInt::zero(); 129];
        // x^128 + x^7 + x^2 + x + 1
        modulus_coeffs[0] = BigInt::one();
        modulus_coeffs[1] = BigInt::one();
        modulus_coeffs[2] = BigInt::one();
        modulus_coeffs[7] = BigInt::one();
        modulus_coeffs[128] = BigInt::one();

        let field = FiniteField::new_extension_field(p, modulus_coeffs);

        // 2. Parsujemy klucz H
        let h_bytes = hex_to_bytes(h_hex);
        let h = bytes_to_gcm_element(&h_bytes, field.clone());

        GHash { field, h }
    }

    /// Implementacja mnożenia GCM na u128.
    /// Reprezentacja: MSB (bit 127) to współczynnik x^0. LSB (bit 0) to x^127.
    /// Mnożenie przez x to przesunięcie w prawo (>> 1).
    fn gcm_mul(&self, x: &FieldElement, y: &FieldElement) -> FieldElement {
        let mut v = poly_to_u128(&x.coeffs);
        let y_int = poly_to_u128(&y.coeffs);
        let mut z: u128 = 0;

        // Stała R: 1 + x + x^2 + x^7
        // Mapowanie: x^0->127, x^1->126, x^2->125, x^7->120
        let r_val: u128 = (1 << 127) | (1 << 126) | (1 << 125) | (1 << 120);

        for i in 0..128 {
            // Sprawdzamy i-ty bit Y (x^i). W u128 x^i jest na pozycji (127 - i).
            if (y_int >> (127 - i)) & 1 == 1 {
                z ^= v;
            }

            // Mnożenie V przez x (shift right w naszej reprezentacji)
            // Sprawdzamy czy spada bit x^127 (LSB)
            let reduced = (v & 1) == 1;
            v >>= 1;

            if reduced {
                v ^= r_val;
            }
        }

        u128_to_poly(z, self.field.clone())
    }

    pub fn calculate(&self, a: &[u8], c: &[u8]) -> String {
        let mut s_bytes = Vec::new();

        // Padding A
        s_bytes.extend_from_slice(a);
        if a.len() % 16 != 0 {
            s_bytes.extend(vec![0u8; 16 - (a.len() % 16)]);
        }

        // Padding C
        s_bytes.extend_from_slice(c);
        if c.len() % 16 != 0 {
            s_bytes.extend(vec![0u8; 16 - (c.len() % 16)]);
        }

        // Lengths (64-bit big endian)
        let a_len_bits = (a.len() as u64) * 8;
        let c_len_bits = (c.len() as u64) * 8;
        s_bytes.extend_from_slice(&a_len_bits.to_be_bytes());
        s_bytes.extend_from_slice(&c_len_bits.to_be_bytes());

        let mut x = FieldElement::new(vec![BigInt::zero()], self.field.clone());

        for chunk in s_bytes.chunks(16) {
            let s_i = bytes_to_gcm_element(chunk, self.field.clone());

            let sum = x + s_i;
            x = self.gcm_mul(&sum, &self.h);
        }

        element_to_gcm_hex(&x)
    }
}

// --- Helper Functions ---

fn poly_to_u128(coeffs: &[BigInt]) -> u128 {
    let mut res: u128 = 0;
    for (i, c) in coeffs.iter().enumerate() {
        if i < 128 && !c.is_zero() {
            // x^i mapujemy na bit (127 - i)
            res |= 1 << (127 - i);
        }
    }
    res
}

fn u128_to_poly(val: u128, field: Rc<FiniteField>) -> FieldElement {
    let mut coeffs = vec![BigInt::zero(); 128];
    for i in 0..128 {
        if (val >> (127 - i)) & 1 == 1 {
            coeffs[i] = BigInt::one();
        }
    }
    // Trim
    while coeffs.len() > 1 && coeffs.last().unwrap().is_zero() {
        coeffs.pop();
    }
    FieldElement::new(coeffs, field)
}

fn hex_to_bytes(s: &str) -> Vec<u8> {
    let clean_s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    (0..clean_s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean_s[i..i + 2], 16).unwrap_or(0))
        .collect()
}

fn bytes_to_gcm_element(bytes: &[u8], field: Rc<FiniteField>) -> FieldElement {
    let mut coeffs = vec![BigInt::zero(); 128];
    for (byte_idx, &byte) in bytes.iter().enumerate() {
        for bit_in_byte in 0..8 {
            let is_set = (byte >> (7 - bit_in_byte)) & 1 == 1;
            if is_set {
                let poly_power = byte_idx * 8 + bit_in_byte;
                if poly_power < 128 {
                    coeffs[poly_power] = BigInt::one();
                }
            }
        }
    }
    while coeffs.len() > 1 && coeffs.last().unwrap().is_zero() {
        coeffs.pop();
    }
    FieldElement::new(coeffs, field)
}

fn element_to_gcm_hex(elem: &FieldElement) -> String {
    let mut bytes = vec![0u8; 16];
    for (power, coeff) in elem.coeffs.iter().enumerate() {
        if !coeff.is_zero() {
            let byte_idx = power / 8;
            let bit_in_byte = power % 8;
            if byte_idx < 16 {
                bytes[byte_idx] |= 1 << (7 - bit_in_byte);
            }
        }
    }
    let mut s = String::with_capacity(32);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}