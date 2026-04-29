use num_bigint::BigInt;
use std::rc::Rc;
use base64::{Engine as _, engine::general_purpose};
use crate::finite_field::{FiniteField, FieldElement};
use crate::elliptic_curve::{EllipticCurve, ECPoint};

// Interfejs do serializacji
pub trait FieldSerde {
    fn to_decimal(&self) -> String;
    fn from_decimal(s: &str, field: Rc<FiniteField>) -> Self;

    fn to_hex(&self) -> String;
    fn from_hex(s: &str, field: Rc<FiniteField>) -> Self;

    fn to_base64(&self) -> String;
    fn from_base64(s: &str, field: Rc<FiniteField>) -> Self;
}

impl FieldSerde for FieldElement {
    // --- DECIMAL (Base 10) ---
    fn to_decimal(&self) -> String {
        // Dla k=1 zwracamy po prostu liczbę. Dla k>1 zwracamy listę [c0, c1, ...]
        if self.field.k == 1 {
            self.coeffs[0].to_str_radix(10)
        } else {
            let parts: Vec<String> = self.coeffs.iter()
                .map(|c| c.to_str_radix(10))
                .collect();
            format!("[{}]", parts.join(", "))
        }
    }

    fn from_decimal(s: &str, field: Rc<FiniteField>) -> Self {
        if field.k == 1 {
            let val = BigInt::parse_bytes(s.as_bytes(), 10).expect("Invalid Decimal");
            FieldElement::new(vec![val], field)
        } else {
            // Oczekujemy formatu "[c0, c1, ...]"
            let content = s.trim_matches(|c| c == '[' || c == ']');
            let coeffs: Vec<BigInt> = content.split(',')
                .map(|sub| BigInt::parse_bytes(sub.trim().as_bytes(), 10).expect("Invalid coeff"))
                .collect();
            FieldElement::new(coeffs, field)
        }
    }

    // --- HEX (Base 16) ---
    fn to_hex(&self) -> String {
        if self.field.k == 1 {
            self.coeffs[0].to_str_radix(16).to_uppercase()
        } else {
            let parts: Vec<String> = self.coeffs.iter()
                .map(|c| c.to_str_radix(16).to_uppercase())
                .collect();
            format!("[{}]", parts.join(", "))
        }
    }

    fn from_hex(s: &str, field: Rc<FiniteField>) -> Self {
        if field.k == 1 {
            let val = BigInt::parse_bytes(s.as_bytes(), 16).expect("Invalid Hex");
            FieldElement::new(vec![val], field)
        } else {
            let content = s.trim_matches(|c| c == '[' || c == ']');
            let coeffs: Vec<BigInt> = content.split(',')
                .map(|sub| BigInt::parse_bytes(sub.trim().as_bytes(), 16).expect("Invalid coeff"))
                .collect();
            FieldElement::new(coeffs, field)
        }
    }

    // --- BASE 64 ---
    fn to_base64(&self) -> String {
        // Konwertujemy BigInt na bajty (Big Endian), a potem na Base64
        // Dla k>1 oddzielamy współczynniki znakiem '|'
        let parts: Vec<String> = self.coeffs.iter()
            .map(|c| general_purpose::STANDARD.encode(c.to_signed_bytes_be()))
            .collect();
        parts.join("|")
    }

    fn from_base64(s: &str, field: Rc<FiniteField>) -> Self {
        let parts: Vec<&str> = s.split('|').collect();
        let coeffs: Vec<BigInt> = parts.iter()
            .map(|sub| {
                let bytes = general_purpose::STANDARD.decode(sub).expect("Invalid Base64");
                BigInt::from_signed_bytes_be(&bytes)
            })
            .collect();
        FieldElement::new(coeffs, field)
    }
}

// Dodajemy helpery do ECPoint (jako metody struktury, bo ECPoint to Enum)
impl ECPoint {
    // Serializuje punkt do formatu "X_STR:Y_STR" (gdzie STR to hex, dec lub b64)
    pub fn serialize(&self, format: &str) -> String {
        match self {
            ECPoint::Infinity { .. } => "INFINITY".to_string(),
            ECPoint::Point { x, y, .. } => {
                let x_s = match format {
                    "dec" => x.to_decimal(),
                    "hex" => x.to_hex(),
                    "b64" => x.to_base64(),
                    _ => panic!("Unknown format"),
                };
                let y_s = match format {
                    "dec" => y.to_decimal(),
                    "hex" => y.to_hex(),
                    "b64" => y.to_base64(),
                    _ => panic!("Unknown format"),
                };
                format!("{}:{}", x_s, y_s)
            }
        }
    }

    pub fn deserialize(s: &str, format: &str, curve: Rc<EllipticCurve>) -> Self {
        if s == "INFINITY" {
            return curve.infinity();
        }

        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            panic!("Invalid point format");
        }

        let field = curve.field.clone();
        let x = match format {
            "dec" => FieldElement::from_decimal(parts[0], field.clone()),
            "hex" => FieldElement::from_hex(parts[0], field.clone()),
            "b64" => FieldElement::from_base64(parts[0], field.clone()),
            _ => panic!("Unknown format"),
        };
        let y = match format {
            "dec" => FieldElement::from_decimal(parts[1], field.clone()),
            "hex" => FieldElement::from_hex(parts[1], field.clone()),
            "b64" => FieldElement::from_base64(parts[1], field.clone()),
            _ => panic!("Unknown format"),
        };

        // Używamy unwrap, bo deserializacja powinna odtwarzać poprawne punkty
        curve.point(x, y).expect("Deserialized point is not on the curve!")
    }
}