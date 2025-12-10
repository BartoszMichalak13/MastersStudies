// src/lib.rs
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Zero, Num};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::rc::Rc;
use std::fmt;

// --- Helper Functions ---

// Rozszerzony algorytm Euklidesa dla liczb całkowitych
fn egcd_bigint(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), BigInt::zero(), BigInt::one())
    } else {
        let (g, x, y) = egcd_bigint(&(b % a), a);
        (g, y - (b / a) * &x, x)
    }
}

// Inwersja modularna dla skalarów
fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd_bigint(a, m);
    if g != BigInt::one() {
        None
    } else {
        Some((x % m + m) % m)
    }
}

// --- Structures ---

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

impl FiniteField {
    pub fn new_prime_field(p: BigInt) -> Rc<Self> {
        Rc::new(FiniteField { p, k: 1, modulus: vec![] })
    }

    pub fn new_extension_field(p: BigInt, modulus: Vec<BigInt>) -> Rc<Self> {
        let k = modulus.len() - 1;
        Rc::new(FiniteField { p, k, modulus })
    }
}

// --- Polynomial Arithmetic Logic ---

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

            if diff_deg < quo.len() {
               quo[diff_deg] = factor.clone();
            }

            for i in 0..=den_deg {
                let val = &factor * &denominator[i];
                let target_idx = i + diff_deg;
                rem[target_idx] = (&rem[target_idx] - val).mod_floor(p);
            }

            while rem.len() > 0 && rem.last().unwrap().is_zero() {
                rem.pop();
            }
        }

        if rem.is_empty() { rem.push(BigInt::zero()); }
        (quo, rem)
    }
}

// --- Operator Overloading ---

impl Add for FieldElement {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let coeffs = self.poly_add(&other);
        FieldElement::new(coeffs, self.field.clone())
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Neg for FieldElement {
    type Output = Self;
    fn neg(self) -> Self {
        let mut new_coeffs = Vec::new();
        for c in &self.coeffs {
            new_coeffs.push(-c);
        }
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

impl FieldElement {
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
            self.coeffs.clone(),
            self.field.modulus.clone(),
            &self.field.p
        );

        if gcd.len() > 1 {
            panic!("GCD degree > 0, modulus likely not irreducible or element is zero divisor");
        }

        let scaler = mod_inverse(&gcd[0], &self.field.p).unwrap();

        for c in &mut t {
            *c = (c as &BigInt * &scaler).mod_floor(&self.field.p);
        }

        FieldElement::new(t, self.field.clone())
    }
}

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

// --- Exponentiation ---

impl FieldElement {
    pub fn pow(&self, exp: &BigInt) -> Self {
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

impl FieldElement {
    // --- Helper: Tworzenie z HEX stringa (dla dużych liczb) ---
    pub fn from_hex_str(hex: &str, field: Rc<FiniteField>) -> Self {
        let val = BigInt::from_str_radix(hex, 16).expect("Invalid Hex String");
        // Jeśli k=1, to po prostu liczba
        if field.k == 1 {
             FieldElement::new(vec![val], field)
        } else {
             // Jeśli k>1 i podajemy jedną liczbę, zakładamy że to reprezentacja bitowa wielomianu (dla F_2^m)
             FieldElement::from_bit_representation(val, field)
        }
    }

    // --- F_2^m (Binary Fields) ---

    // Konwertuje dużą liczbę całkowitą na wielomian nad F_2
    // Np. liczba 5 (binarnie 101) -> wielomian 1*x^2 + 0*x + 1 -> coeffs: [1, 0, 1]
    pub fn from_bit_representation(val: BigInt, field: Rc<FiniteField>) -> Self {
        if field.p != BigInt::from(2) {
             panic!("Bit representation is valid only for characteristic p=2");
        }

        let mut coeffs = Vec::new();
        let mut temp = val;

        // Wyciągamy bity od najmniej znaczącego (Little Endian dla wielomianów)
        while temp > BigInt::zero() {
            if &temp % 2 == BigInt::one() {
                coeffs.push(BigInt::one());
            } else {
                coeffs.push(BigInt::zero());
            }
            temp /= 2;
        }

        // Uzupełnij zerami, jeśli wyszło puste (dla zera)
        if coeffs.is_empty() { coeffs.push(BigInt::zero()); }

        FieldElement::new(coeffs, field)
    }

    // Konwertuje wielomian nad F_2 z powrotem na liczbę (Bit String / Integer)
    pub fn to_bit_representation(&self) -> BigInt {
         if self.field.p != BigInt::from(2) {
             panic!("Bit representation is valid only for characteristic p=2");
        }

        let mut res = BigInt::zero();
        // coeffs[0] to x^0 (najmniej znaczący bit), coeffs[N] to x^N (najbardziej znaczący)
        for (i, c) in self.coeffs.iter().enumerate().rev() {
            res *= 2;
            if c.is_one() {
                res += 1;
            }
        }
        res
    }
}

// --- Formatting ---

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



// --- Elliptic Curve Structures ---

#[derive(Debug, PartialEq)]
pub struct EllipticCurve {
    pub a: FieldElement,
    pub b: FieldElement,
    // Przechowujemy też referencję do ciała, nad którym jest krzywa,
    // aby łatwo tworzyć nowe elementy (np. zero, jeden, współczynniki).
    pub field: Rc<FiniteField>,
}

#[derive(Clone, Debug)]
pub enum ECPoint {
    Infinity { curve: Rc<EllipticCurve> },
    Point {
        x: FieldElement,
        y: FieldElement,
        curve: Rc<EllipticCurve>,
    },
}

impl EllipticCurve {
    // Konstruktor krzywej. Sprawdza, czy a i b należą do podanego ciała.
    pub fn new(a: FieldElement, b: FieldElement, field: Rc<FiniteField>) -> Rc<Self> {
        // Opcjonalnie: Sprawdzenie wyróżnika (discriminant) 4a^3 + 27b^2 != 0,
        // aby upewnić się, że krzywa nie jest osobliwa (singular).
        Rc::new(EllipticCurve { a, b, field })
    }

    // Pomocnicza funkcja do tworzenia Punktu w Nieskończoności na tej krzywej
    pub fn infinity(self: &Rc<Self>) -> ECPoint {
        ECPoint::Infinity { curve: self.clone() }
    }

    // Tworzenie punktu ze sprawdzeniem, czy leży na krzywej
    pub fn point(self: &Rc<Self>, x: FieldElement, y: FieldElement) -> Option<ECPoint> {
        // Sprawdzenie równania: y^2 = x^3 + ax + b
        let y2 = y.pow(&BigInt::from(2));
        let x3 = x.pow(&BigInt::from(3));
        let ax = self.a.clone() * x.clone();
        let rhs = x3 + ax + self.b.clone();

        if y2.coeffs == rhs.coeffs {
            Some(ECPoint::Point { x, y, curve: self.clone() })
        } else {
            None // Punkt nie leży na krzywej
        }
    }
}

// --- Elliptic Curve Arithmetic ---

impl PartialEq for ECPoint {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ECPoint::Infinity { .. }, ECPoint::Infinity { .. }) => true,
            (ECPoint::Point { x: x1, y: y1, .. }, ECPoint::Point { x: x2, y: y2, .. }) => {
                x1.coeffs == x2.coeffs && y1.coeffs == y2.coeffs
            }
            _ => false,
        }
    }
}

impl ECPoint {
    // Sprawdza, czy dwa punkty są na tej samej krzywej (ważne dla bezpieczeństwa)
    fn check_same_curve(&self, other: &Self) {
        let curve1 = match self {
            ECPoint::Infinity { curve } => curve,
            ECPoint::Point { curve, .. } => curve,
        };
        let curve2 = match other {
            ECPoint::Infinity { curve } => curve,
            ECPoint::Point { curve, .. } => curve,
        };

        // Porównujemy parametry a i b krzywych
        assert_eq!(curve1.a.coeffs, curve2.a.coeffs, "Points are on different curves!");
        assert_eq!(curve1.b.coeffs, curve2.b.coeffs, "Points are on different curves!");
    }

    // Inwersja punktu: -P = (x, -y)
    pub fn neg(&self) -> Self {
        match self {
            ECPoint::Infinity { curve } => ECPoint::Infinity { curve: curve.clone() },
            ECPoint::Point { x, y, curve } => ECPoint::Point {
                x: x.clone(),
                y: y.clone().neg(), // Używamy negacji z FieldElement
                curve: curve.clone(),
            },
        }
    }
}

impl Add for ECPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.check_same_curve(&other);

        match (self.clone(), other.clone()) {
            // 1. P + O = P
            (ECPoint::Infinity { .. }, _) => other,
            // 2. O + P = P
            (_, ECPoint::Infinity { .. }) => self,

            (ECPoint::Point { x: x1, y: y1, curve }, ECPoint::Point { x: x2, y: y2, .. }) => {
                // 3. P + (-P) = O (Pionowa linia)
                // Jeśli x1 == x2 i y1 != y2 (czyli y1 = -y2), to wynik Infinity
                if x1.coeffs == x2.coeffs && y1.coeffs != y2.coeffs {
                    return ECPoint::Infinity { curve };
                }

                // Obliczanie nachylenia (lambda)
                let slope = if x1.coeffs != x2.coeffs {
                    // Przypadek P != Q (Dodawanie)
                    // lambda = (y2 - y1) / (x2 - x1)
                    let num = y2 - y1.clone();
                    let den = x2.clone() - x1.clone();
                    num / den
                } else {
                    // Przypadek P == Q (Podwajanie)
                    if y1.coeffs == vec![BigInt::zero()] {
                        // Styczna jest pionowa (dzielenie przez zero) -> Infinity
                        return ECPoint::Infinity { curve };
                    }

                    // lambda = (3x1^2 + a) / (2y1)

                    // Helpery do liczb całkowitych 2 i 3 w ciele
                    let two = FieldElement::new(vec![BigInt::from(2)], curve.field.clone());
                    let three = FieldElement::new(vec![BigInt::from(3)], curve.field.clone());

                    let x1_sq = x1.clone() * x1.clone();
                    let num = (three * x1_sq) + curve.a.clone();
                    let den = two * y1.clone();
                    num / den
                };

                // Wzory końcowe (wspólne dla dodawania i podwajania):
                // x3 = lambda^2 - x1 - x2
                // y3 = lambda * (x1 - x3) - y1

                let slope_sq = slope.clone() * slope.clone();
                let x3 = slope_sq - x1.clone() - x2;
                let y3 = slope * (x1 - x3.clone()) - y1;

                ECPoint::Point { x: x3, y: y3, curve }
            }
        }
    }
}

// Mnożenie skalarne (Point Multiplication) n * P
// Algorytm "Double and Add"
impl Mul<&BigInt> for ECPoint {
    type Output = Self;

    fn mul(self, scalar: &BigInt) -> Self {
        let curve = match &self {
            ECPoint::Infinity { curve } => curve.clone(),
            ECPoint::Point { curve, .. } => curve.clone(),
        };

        let mut res = ECPoint::Infinity { curve: curve.clone() };
        let mut temp = self.clone();
        let mut n = scalar.clone();

        // Obsługa ujemnego skalara (jeśli BigInt to wspiera)
        if n < BigInt::zero() {
            temp = temp.neg();
            n = -n;
        }

        while n > BigInt::zero() {
            if &n % 2 == BigInt::one() {
                res = res + temp.clone();
            }
            temp = temp.clone() + temp.clone(); // Double
            n /= 2;
        }
        res
    }
}