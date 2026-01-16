use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero, Num};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;
use std::fmt;

// --- Helper Functions ---
fn egcd_bigint(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = egcd_bigint(&(b % a), a);
        (g, y - (b / a) * &x, x)
    }
}

fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd_bigint(a, m);
    if g != BigInt::one() {
        None
    } else {
        Some((x % m + m) % m)
    }
}

// --- Finite Field Structures ---

#[derive(Debug, PartialEq)]
pub struct FiniteField {
    pub p: BigInt,
    pub k: usize,
    pub modulus: Vec<BigInt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldElement {
    pub coeffs: Vec<BigInt>,
    pub field: Rc<FiniteField>,
}

// --- Implementations ---

impl FiniteField {
    pub fn new_prime_field(p: BigInt) -> Rc<Self> {
        Rc::new(FiniteField { p, k: 1, modulus: vec![] })
    }

    pub fn new_extension_field(p: BigInt, modulus: Vec<BigInt>) -> Rc<Self> {
        let k = modulus.len() - 1;
        Rc::new(FiniteField { p, k, modulus })
    }
}

impl FieldElement {
    pub fn new(coeffs: Vec<BigInt>, field: Rc<FiniteField>) -> Self {
        let mut element = FieldElement { coeffs, field };
        element.normalize();
        element
    }
    
    fn normalize(&mut self) {
        let p = &self.field.p;
        for c in &mut self.coeffs {
            *c = c.mod_floor(p);
        }
        while self.coeffs.len() > 1 && self.coeffs.last().unwrap().is_zero() {
            self.coeffs.pop();
        }
    }

    pub fn from_hex_str(hex: &str, field: Rc<FiniteField>) -> Self {
        let val = BigInt::from_str_radix(hex, 16).expect("Invalid Hex String");
        if field.k == 1 {
             FieldElement::new(vec![val], field)
        } else {
             FieldElement::from_bit_representation(val, field)
        }
    }

    pub fn from_bit_representation(val: BigInt, field: Rc<FiniteField>) -> Self {
        if field.p != BigInt::from(2) {
             panic!("Bit representation is valid only for characteristic p=2");
        }
        let mut coeffs = Vec::new();
        let mut temp = val;
        while temp > BigInt::zero() {
            if &temp % 2 == BigInt::one() {
                coeffs.push(BigInt::one());
            } else {
                coeffs.push(BigInt::zero());
            }
            temp /= 2;
        }
        if coeffs.is_empty() { coeffs.push(BigInt::zero()); }
        FieldElement::new(coeffs, field)
    }

    pub fn to_bit_representation(&self) -> BigInt {
         if self.field.p != BigInt::from(2) {
             panic!("Bit representation is valid only for characteristic p=2");
        }
        let mut res = BigInt::zero();
        for (_, c) in self.coeffs.iter().enumerate().rev() {
            res *= 2;
            if c.is_one() { res += 1; }
        }
        res
    }

    fn poly_add(&self, other: &Self) -> Vec<BigInt> {
        let max_len = std::cmp::max(self.coeffs.len(), other.coeffs.len());
        let mut result = Vec::with_capacity(max_len);
        let zero = BigInt::zero(); 
        for i in 0..max_len {
            let a = self.coeffs.get(i).unwrap_or(&zero);
            let b = other.coeffs.get(i).unwrap_or(&zero);
            result.push(a + b);
        }
        result
    }

    fn poly_mul_raw(&self, other: &Self) -> Vec<BigInt> {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return vec![BigInt::zero()];
        }
        let deg_a = self.coeffs.len() - 1;
        let deg_b = other.coeffs.len() - 1;
        let mut result = vec![BigInt::zero(); deg_a + deg_b + 1];

        for (i, c1) in self.coeffs.iter().enumerate() {
            for (j, c2) in other.coeffs.iter().enumerate() {
                result[i + j] += c1 * c2;
            }
        }
        result
    }

    fn poly_div_rem(numerator: &[BigInt], denominator: &[BigInt], p: &BigInt) -> (Vec<BigInt>, Vec<BigInt>) {
        let mut rem = numerator.to_vec();
        let den_deg = denominator.len() - 1;
        let den_lead = denominator.last().unwrap();
        
        let den_lead_inv = mod_inverse(den_lead, p).expect("Leading coefficient not invertible in F_p");
        let mut quo = vec![BigInt::zero(); if rem.len() > den_deg { rem.len() - den_deg } else { 1 }];

        while rem.len() >= denominator.len() {
            let rem_deg = rem.len() - 1;
            let diff_deg = rem_deg - den_deg;
            let rem_lead = rem.last().unwrap().clone();
            let factor = (&rem_lead * &den_lead_inv).mod_floor(p);
            
            if diff_deg < quo.len() { quo[diff_deg] = factor.clone(); }

            for i in 0..=den_deg {
                let val = &factor * &denominator[i];
                let target_idx = i + diff_deg;
                rem[target_idx] = (&rem[target_idx] - val).mod_floor(p);
            }
            while rem.len() > 0 && rem.last().unwrap().is_zero() { rem.pop(); }
        }
        if rem.is_empty() { rem.push(BigInt::zero()); }
        (quo, rem)
    }

    fn extended_euclidean_poly(a: Vec<BigInt>, b: Vec<BigInt>, p: &BigInt) -> (Vec<BigInt>, Vec<BigInt>, Vec<BigInt>) {
        let mut r0 = a;
        let mut r1 = b;
        let mut t0 = vec![BigInt::one()]; 
        let mut t1 = vec![BigInt::zero()]; 
        let s0 = vec![BigInt::zero()]; 

        while !(r1.len() == 1 && r1[0].is_zero()) && !r1.is_empty() {
             let (quo, rem) = FieldElement::poly_div_rem(&r0, &r1, p);
             r0 = r1;
             r1 = rem;
             
             let quo_times_t1 = {
                 let mut res = vec![BigInt::zero(); quo.len() + t1.len()]; 
                 if !quo.is_empty() && !t1.is_empty() {
                    res = vec![BigInt::zero(); quo.len() + t1.len() - 1]; 
                     for (i, c1) in quo.iter().enumerate() {
                        for (j, c2) in t1.iter().enumerate() {
                            res[i+j] = (&res[i+j] + c1 * c2) % p;
                        }
                     }
                 }
                 while res.len() > 1 && res.last().unwrap().is_zero() { res.pop(); }
                 res
             };
             
             let t_next = {
                 let max_len = std::cmp::max(t0.len(), quo_times_t1.len());
                 let mut res = Vec::with_capacity(max_len);
                 let zero = BigInt::zero();
                 for i in 0..max_len {
                     let v1 = t0.get(i).unwrap_or(&zero);
                     let v2 = quo_times_t1.get(i).unwrap_or(&zero);
                     res.push((v1 - v2).mod_floor(p));
                 }
                 while res.len() > 1 && res.last().unwrap().is_zero() { res.pop(); }
                 res
             };
             t0 = t1;
             t1 = t_next;
        }
        (r0, t0, s0)
    }

    pub fn inverse(self) -> Self {
        if self.coeffs.len() == 1 && self.coeffs[0].is_zero() {
            panic!("Cannot invert zero");
        }
        if self.field.k == 1 {
            let inv = mod_inverse(&self.coeffs[0], &self.field.p).expect("Inverse failed");
            return FieldElement::new(vec![inv], self.field.clone());
        }
        let (gcd, mut t, _) = FieldElement::extended_euclidean_poly(
            self.coeffs.clone(), self.field.modulus.clone(), &self.field.p
        );
        let scaler = mod_inverse(&gcd[0], &self.field.p).unwrap();
        for c in &mut t {
            *c = (c as &BigInt * &scaler).mod_floor(&self.field.p);
        }
        FieldElement::new(t, self.field.clone())
    }
    
    // --- Constant-Time Exponentiation (Montgomery Ladder) ---
    pub fn pow(&self, exp: &BigInt) -> Self {
        // R0 = 1 (akumulator), R1 = a (podstawa)
        let mut r0 = FieldElement::new(vec![BigInt::one()], self.field.clone());
        let mut r1 = self.clone();

        let num_bits = exp.bits();

        for i in (0..num_bits).rev() {
            let bit = exp.bit(i);

            // CSWAP: Jeśli bit 1, zamień role rejestrów
            if bit {
                std::mem::swap(&mut r0, &mut r1);
            }

            // Operacje drabiny:
            // R1 = R0 * R1 (Mnożenie)
            // R0 = R0 * R0 (Kwadratowanie)

            let r0_sq = r0.clone() * r0.clone();
            let prod = r0.clone() * r1.clone();

            r0 = r0_sq;
            r1 = prod;

            // CSWAP: Odwróć zamianę
            if bit {
                std::mem::swap(&mut r0, &mut r1);
            }
        }
        r0
    }

    // Old Pow Implementation (not used)
    pub fn old_pow(&self, exp: &BigInt) -> Self {
        let mut res = FieldElement::new(vec![BigInt::one()], self.field.clone());
        let mut base = self.clone();
        let mut e = exp.clone();
        if e < BigInt::zero() {
            base = base.inverse();
            e = -e;
        }
        while e > BigInt::zero() {
            if &e % 2 == BigInt::one() {
                res = res * base.clone();
            }
            base = base.clone() * base;
            e /= 2;
        }
        res
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let coeffs = self.poly_add(&other);
        FieldElement::new(coeffs, self.field.clone())
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self { self + (-other) }
}

impl Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self {
        let mut new_coeffs = Vec::new();
        for c in &self.coeffs { new_coeffs.push(-c); }
        FieldElement::new(new_coeffs, self.field.clone())
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let raw_prod = self.poly_mul_raw(&other);
        if self.field.k == 1 {
             FieldElement::new(raw_prod, self.field.clone())
        } else {
            let (_, rem) = FieldElement::poly_div_rem(&raw_prod, &self.field.modulus, &self.field.p);
            FieldElement::new(rem, self.field.clone())
        }
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self { self * rhs.inverse() }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coeffs.is_empty() || (self.coeffs.len() == 1 && self.coeffs[0].is_zero()) {
            return write!(f, "0");
        }
        let mut first = true;
        for (i, c) in self.coeffs.iter().enumerate().rev() {
            if c.is_zero() { continue; }
            if !first { write!(f, " + ")?; }
            if i == 0 {
                write!(f, "{}", c)?;
            } else if i == 1 {
                 if c.is_one() { write!(f, "x")?; } else { write!(f, "{}x", c)?; }
            } else {
                 if c.is_one() { write!(f, "x^{}", i)?; } else { write!(f, "{}x^{}", c, i)?; }
            }
            first = false;
        }
        Ok(())
    }
}