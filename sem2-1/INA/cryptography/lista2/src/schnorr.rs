use num_bigint::{BigInt, RandBigInt, Sign};
use num_traits::{One};
use sha2::{Sha256, Digest};
use crate::finite_field::FieldElement;
use crate::elliptic_curve::ECPoint;
use crate::schnorr_encoding::{encode_r, SchnorrR};

pub struct SchnorrSignature {
    pub s: BigInt,
    pub e: BigInt,
}

// Funkcja pomocnicza: H(R || m) -> BigInt
fn hash_r_m(r_encoded: &str, message: &str, order: &BigInt) -> BigInt {
    // e = H(R || m)
    // "Convert the encoded value to a compact JSON form... then hash"
    let input = format!("{}{}", r_encoded, message);
    
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    
    // Interpretujemy hash jako BigInt (Big Endian)
    let e_raw = BigInt::from_bytes_be(Sign::Plus, &result);
    
    // e musi być w Z_q
    e_raw % order
}

// --- Wariant dla Ciał Skończonych (F_p, F_p^k) ---
pub struct SchnorrField {
    pub g: FieldElement,
    pub order: BigInt, // q
}

impl SchnorrField {
    pub fn sign(&self, msg: &str, sk: &BigInt) -> SchnorrSignature {
        let mut rng = rand::thread_rng();
        // 1. k <- random [1, q-1]
        let k = rng.gen_bigint_range(&BigInt::one(), &self.order);
        
        // 2. R = g^k
        let r_val = self.g.pow(&k);
        let r_encoded = encode_r(&SchnorrR::Field(r_val));
        
        // 3. e = H(R || m)
        let e = hash_r_m(&r_encoded, msg, &self.order);
        
        // 4. s = (k - x*e) mod q
        let xe = (sk * &e) % &self.order;
        let s = (&k - xe + &self.order) % &self.order; // Dodajemy order, żeby wynik był dodatni
        
        SchnorrSignature { s, e }
    }

    pub fn verify(&self, msg: &str, pk: &FieldElement, sig: &SchnorrSignature) -> bool {
        // 1. R_rec = g^s * y^e
        // y to klucz publiczny (g^x)
        let gs = self.g.pow(&sig.s);
        let ye = pk.pow(&sig.e);
        let r_rec = gs * ye;
        
        // 2. e_rec = H(R_rec || m)
        let r_encoded = encode_r(&SchnorrR::Field(r_rec));
        let e_rec = hash_r_m(&r_encoded, msg, &self.order);
        
        // 3. Check e == e_rec
        sig.e == e_rec
    }
}

// --- Wariant dla Krzywych Eliptycznych ---
pub struct SchnorrCurve {
    pub g: ECPoint,
    pub order: BigInt,
}

impl SchnorrCurve {
    pub fn sign(&self, msg: &str, sk: &BigInt) -> SchnorrSignature {
        let mut rng = rand::thread_rng();
        // 1. k <- random [1, q-1]
        let k = rng.gen_bigint_range(&BigInt::one(), &self.order);
        
        // 2. R = k * G
        let r_point = self.g.clone() * &k;
        let r_encoded = encode_r(&SchnorrR::Curve(r_point));
        
        // 3. e = H(R || m)
        let e = hash_r_m(&r_encoded, msg, &self.order);
        
        // 4. s = (k - x*e) mod q
        let xe = (sk * &e) % &self.order;
        let s = (&k - xe + &self.order) % &self.order;
        
        SchnorrSignature { s, e }
    }

    pub fn verify(&self, msg: &str, pk: &ECPoint, sig: &SchnorrSignature) -> bool {
        // 1. R_rec = s*G + e*PK
        let sg = self.g.clone() * &sig.s;
        let epk = pk.clone() * &sig.e;
        let r_rec = sg + epk; // Dodawanie punktów
        
        // 2. e_rec = H(R_rec || m)
        let r_encoded = encode_r(&SchnorrR::Curve(r_rec));
        let e_rec = hash_r_m(&r_encoded, msg, &self.order);
        
        sig.e == e_rec
    }
}