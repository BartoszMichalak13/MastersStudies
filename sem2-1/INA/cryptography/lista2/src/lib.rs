pub mod finite_field;
pub mod elliptic_curve;
pub mod serialization;
pub mod ghash;
pub mod schnorr_encoding;
pub mod schnorr;

// Re-eksportujemy struktury na główny poziom (opcjonalne, ale wygodne)
// Dzięki temu piszemy: lista2::FiniteField zamiast lista2::finite_field::FiniteField
pub use finite_field::{FiniteField, FieldElement};
pub use elliptic_curve::{EllipticCurve, ECPoint};
pub use serialization::FieldSerde;
pub use ghash::GHash;
pub use schnorr::{SchnorrField, SchnorrCurve, SchnorrSignature};
pub use schnorr_encoding::{encode_r, SchnorrR};