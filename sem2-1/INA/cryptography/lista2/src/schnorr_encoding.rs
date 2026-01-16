use crate::finite_field::FieldElement;
use crate::elliptic_curve::ECPoint;
use num_bigint::BigInt;
use num_traits::Zero;
use serde_json::json;

/// Pomocnicza funkcja do formatowania BigInt jako hex z paddingiem.
pub fn encode_scalar(val: &BigInt, bits_len: u64) -> String {
    // Obliczamy ile bajtów (zaokrąglone w górę)
    let byte_len = (bits_len + 7) / 8;
    
    // Zamieniamy na hex
    let raw_hex = val.to_str_radix(16).to_uppercase();
    
    // Padding zerami
    let target_char_len = (byte_len * 2) as usize;
    if raw_hex.len() >= target_char_len {
        raw_hex
    } else {
        let padding = "0".repeat(target_char_len - raw_hex.len());
        format!("{}{}", padding, raw_hex)
    }
}

/// Helper, który decyduje jak zakodować element ciała (String czy Array)
/// i zwraca go jako serde_json::Value.
fn encode_element_to_json(elem: &FieldElement) -> serde_json::Value {
    // Sprawdzamy charakterystykę ciała
    if elem.field.p == BigInt::from(2) {
        // --- Przypadek (b): F_2^k (Characteristic 2) ---
        // "encode it as if the bit string were an integer"
        // Zamieniamy wielomian (wektor bitów) na jedną liczbę BigInt
        let mut val = BigInt::zero();
        for (i, c) in elem.coeffs.iter().enumerate() {
            if !c.is_zero() {
                val.set_bit(i as u64, true);
            }
        }
        // Długość w bitach to stopień rozszerzenia k (oznaczone jako m w zadaniu)
        let m_bits = elem.field.k as u64;
        let hex_str = encode_scalar(&val, m_bits);
        json!(hex_str)
    } else if elem.field.k == 1 {
        // --- Przypadek (a): F_p (Prime field) ---
        // Pojedyncza liczba
        let p_bits = elem.field.p.bits();
        let val = if elem.coeffs.is_empty() { &BigInt::zero() } else { &elem.coeffs[0] };
        let hex_str = encode_scalar(val, p_bits);
        json!(hex_str)
    } else {
        // --- Przypadek (c): F_p^k (Extended prime field, p > 2) ---
        // Tablica współczynników
        let p_bits = elem.field.p.bits();
        let mut parts = Vec::new();
        // Iterujemy po wszystkich współczynnikach od 0 do k-1
        for i in 0..elem.field.k {
            let c = if i < elem.coeffs.len() { &elem.coeffs[i] } else { &BigInt::zero() };
            parts.push(encode_scalar(c, p_bits));
        }
        json!(parts)
    }
}

/// Główna funkcja kodująca element R (FieldElement lub ECPoint) do stringa JSON.
pub fn encode_r(r: &SchnorrR) -> String {
    match r {
        SchnorrR::Field(elem) => {
            // Wywołujemy helper i zamieniamy wynik na string
            // Jeśli to string (F_p, F_2^m), to to_string() doda cudzysłowy (np. "000011")
            // Jeśli to tablica (F_p^k), to_string() zrobi [ "...", "..." ]
            encode_element_to_json(elem).to_string()
        },
        SchnorrR::Curve(point) => {
            // --- Przypadek (d): Elliptic Curve Point ---
            // "encode each coordinate independently according to the rules above"
            match point {
                ECPoint::Infinity { .. } => "\"INFINITY\"".to_string(),
                ECPoint::Point { x, y, .. } => {
                    let x_json = encode_element_to_json(x);
                    let y_json = encode_element_to_json(y);
                    
                    let json_obj = json!({
                        "x": x_json,
                        "y": y_json
                    });
                    // Compact JSON (bez spacji)
                    json_obj.to_string()
                }
            }
        }
    }
}

pub enum SchnorrR {
    Field(FieldElement),
    Curve(ECPoint),
}