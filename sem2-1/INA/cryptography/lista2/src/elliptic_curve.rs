use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::rc::Rc;
use std::ops::{Add, Mul, Neg};
use crate::finite_field::{FiniteField, FieldElement};

#[derive(Debug, PartialEq)]
pub struct EllipticCurve {
    pub a: FieldElement,
    pub b: FieldElement,
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
    pub fn new(a: FieldElement, b: FieldElement, field: Rc<FiniteField>) -> Rc<Self> {
        Rc::new(EllipticCurve { a, b, field })
    }

    pub fn is_binary(&self) -> bool {
        self.field.p == BigInt::from(2)
    }

    pub fn infinity(self: &Rc<Self>) -> ECPoint {
        ECPoint::Infinity { curve: self.clone() }
    }

    pub fn point(self: &Rc<Self>, x: FieldElement, y: FieldElement) -> Option<ECPoint> {
        if self.is_binary() {
            let y2 = y.pow(&BigInt::from(2));
            let xy = x.clone() * y.clone();
            let lhs = y2 + xy;

            let x2 = x.pow(&BigInt::from(2));
            let x3 = x.clone() * x2.clone();
            let ax2 = self.a.clone() * x2;
            let rhs = x3 + ax2 + self.b.clone();

            if lhs.coeffs == rhs.coeffs {
                 Some(ECPoint::Point { x, y, curve: self.clone() })
            } else {
                 None
            }
        } else {
            let y2 = y.pow(&BigInt::from(2));
            let x3 = x.pow(&BigInt::from(3));
            let ax = self.a.clone() * x.clone();
            let rhs = x3 + ax + self.b.clone();

            if y2.coeffs == rhs.coeffs {
                Some(ECPoint::Point { x, y, curve: self.clone() })
            } else {
                None
            }
        }
    }
}

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
    pub fn get_curve(&self) -> Rc<EllipticCurve> {
        match self {
            ECPoint::Infinity { curve } => curve.clone(),
            ECPoint::Point { curve, .. } => curve.clone(),
        }
    }

    fn check_same_curve(&self, other: &Self) {
        let curve1 = match self {
            ECPoint::Infinity { curve } => curve,
            ECPoint::Point { curve, .. } => curve,
        };
        let curve2 = match other {
            ECPoint::Infinity { curve } => curve,
            ECPoint::Point { curve, .. } => curve,
        };
        assert_eq!(curve1.a.coeffs, curve2.a.coeffs, "Points on diff curves");
        assert_eq!(curve1.b.coeffs, curve2.b.coeffs, "Points on diff curves");
        assert_eq!(curve1.field.p, curve2.field.p, "Points on diff fields");
    }

    pub fn neg(&self) -> Self {
        match self {
            ECPoint::Infinity { curve } => ECPoint::Infinity { curve: curve.clone() },
            ECPoint::Point { x, y, curve } => {
                if curve.is_binary() {
                    let new_y = x.clone() + y.clone();
                    ECPoint::Point {
                        x: x.clone(),
                        y: new_y,
                        curve: curve.clone(),
                    }
                } else {
                    ECPoint::Point {
                        x: x.clone(),
                        y: y.clone().neg(),
                        curve: curve.clone(),
                    }
                }
            },
        }
    }
}

impl Add for ECPoint {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.check_same_curve(&other);

        match (self.clone(), other.clone()) {
            (ECPoint::Infinity { .. }, _) => other,
            (_, ECPoint::Infinity { .. }) => self,
            
            (ECPoint::Point { x: x1, y: y1, curve }, ECPoint::Point { x: x2, y: y2, .. }) => {
                
                if curve.is_binary() {
                    if x1.coeffs == x2.coeffs {
                        if y1.coeffs == y2.coeffs {
                            // Doubling in F_2^m
                            if x1.coeffs == vec![BigInt::zero()] {
                                return ECPoint::Infinity { curve };
                            }

                            let lambda = x1.clone() + (y1.clone() / x1.clone());
                            let lambda_sq = lambda.clone() * lambda.clone();
                            let x3 = lambda_sq.clone() + lambda.clone() + curve.a.clone();
                            
                            let one = FieldElement::new(vec![BigInt::one()], curve.field.clone());
                            let x1_sq = x1.clone() * x1.clone();
                            let y3 = x1_sq + ((lambda + one) * x3.clone());

                            return ECPoint::Point { x: x3, y: y3, curve };

                        } else {
                            return ECPoint::Infinity { curve };
                        }
                    } else {
                        // Addition in F_2^m
                        let num = y1.clone() + y2.clone();
                        let den = x1.clone() + x2.clone();
                        let lambda = num / den;
                        
                        let lambda_sq = lambda.clone() * lambda.clone();
                        let a = curve.a.clone();
                        
                        let x3 = lambda_sq + lambda.clone() + x1.clone() + x2.clone() + a;
                        
                        let term1 = lambda * (x1.clone() + x3.clone());
                        let y3 = term1 + x3.clone() + y1.clone();
                        
                        return ECPoint::Point { x: x3, y: y3, curve };
                    }
                }
                else {
                    // Short Weierstrass
                    if x1.coeffs == x2.coeffs && y1.coeffs != y2.coeffs {
                        return ECPoint::Infinity { curve };
                    }

                    let slope = if x1.coeffs != x2.coeffs {
                        let num = y2 - y1.clone();
                        let den = x2.clone() - x1.clone();
                        num / den
                    } else {
                        if y1.coeffs == vec![BigInt::zero()] {
                            return ECPoint::Infinity { curve };
                        }
                        let two = FieldElement::new(vec![BigInt::from(2)], curve.field.clone());
                        let three = FieldElement::new(vec![BigInt::from(3)], curve.field.clone());

                        let x1_sq = x1.clone() * x1.clone();
                        let num = (three * x1_sq) + curve.a.clone();
                        let den = two * y1.clone();
                        num / den
                    };

                    let slope_sq = slope.clone() * slope.clone();
                    let x3 = slope_sq - x1.clone() - x2;
                    let y3 = slope * (x1 - x3.clone()) - y1;

                    ECPoint::Point { x: x3, y: y3, curve }
                }
            }
        }
    }
}

impl Mul<&BigInt> for ECPoint {
    type Output = Self;

    fn mul(self, scalar: &BigInt) -> Self {
        // Obsługa ujemnego skalara
        let (n, point) = if scalar < &BigInt::zero() {
            (-scalar, self.neg())
        } else {
            (scalar.clone(), self.clone())
        };

        // R0 = O (Infinity), R1 = P
        let mut r0 = ECPoint::Infinity { curve: self.get_curve() };
        let mut r1 = point;

        let num_bits = n.bits();

        for i in (0..num_bits).rev() {
            let bit = n.bit(i);

            // CSWAP (Conditional Swap)
            if bit {
                std::mem::swap(&mut r0, &mut r1);
            }
            // Poprawny algorytm Montgomery Ladder z CSWAP:
            // 1. Swap jeśli bit=1.
            // 2. r1 = r0 + r1
            // 3. r0 = 2 * r0
            // 4. Swap jeśli bit=1.

            let sum = r0.clone() + r1.clone();
            let double = r0.clone() + r0.clone(); // 2 * R0

            r1 = sum;
            r0 = double;

            if bit {
                std::mem::swap(&mut r0, &mut r1);
            }
        }
        r0
    }
}

// // Old implementation without constant-time considerations
// impl Mul<&BigInt> for ECPoint {
//     type Output = Self;

//     fn mul(self, scalar: &BigInt) -> Self {
//         let curve = match &self {
//             ECPoint::Infinity { curve } => curve.clone(),
//             ECPoint::Point { curve, .. } => curve.clone(),
//         };

//         let mut res = ECPoint::Infinity { curve: curve.clone() };
//         let mut temp = self.clone();
//         let mut n = scalar.clone();

//         if n < BigInt::zero() {
//             temp = temp.neg();
//             n = -n;
//         }

//         while n > BigInt::zero() {
//             if &n % 2 == BigInt::one() {
//                 res = res + temp.clone();
//             }
//             temp = temp.clone() + temp.clone();
//             n /= 2;
//         }
//         res
//     }
// }