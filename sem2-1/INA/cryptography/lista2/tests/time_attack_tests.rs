use num_bigint::BigInt;
use std::time::Instant;
use std::rc::Rc;
use lista2::{FiniteField, FieldElement};

fn get_large_field() -> Rc<FiniteField> {
    // Duże ciało p ~ 2^2048 (żeby obliczenia trwały chwilę i różnice były widoczne)
    // Użyjmy mniejszego 2^256 dla szybkości testu, ale powtórzmy operację wiele razy.
    let p_hex = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F"; 
    let p = BigInt::parse_bytes(p_hex.as_bytes(), 16).unwrap();
    FiniteField::new_prime_field(p)
}

#[test]
fn test_constant_time_pow() {
    let field = get_large_field();
    let base = FieldElement::new(vec![BigInt::from(12345)], field.clone());

    // 1. Hamming Weight = 1
    // Np. 2^255 (jeden bit ustawiony na początku)
    let exp_light = BigInt::from(1) << 255;

    // 2. Hamming Weight = 256
    // Same jedynki
    let exp_heavy = (BigInt::from(1) << 256) - 1;

    let _ = base.old_pow(&exp_light);

    let start = Instant::now();
    for _ in 0..100 {
        let _ = base.old_pow(&exp_light);
    }
    let duration_light = start.elapsed();

    let start = Instant::now();
    for _ in 0..100 {
        let _ = base.old_pow(&exp_heavy);
    }
    let duration_heavy = start.elapsed();

    println!("Light exponent old pow (100 ops): {:?}", duration_light);
    println!("Heavy exponent old pow (100 ops): {:?}", duration_heavy);

    let diff = if duration_light > duration_heavy {
        duration_light.as_millis() - duration_heavy.as_millis()
    } else {
        duration_heavy.as_millis() - duration_light.as_millis()
    };

    let avg = (duration_light.as_millis() + duration_heavy.as_millis()) / 2;
    let percent_diff = (diff as f64 / avg as f64) * 100.0;

    println!("Różnica czasu old pow: {:.2}%", percent_diff);




    let _ = base.pow(&exp_light);

    let start = Instant::now();
    for _ in 0..100 {
        let _ = base.pow(&exp_light);
    }
    let duration_light = start.elapsed();

    let start = Instant::now();
    for _ in 0..100 {
        let _ = base.pow(&exp_heavy);
    }
    let duration_heavy = start.elapsed();

    println!("Light exponent (100 ops): {:?}", duration_light);
    println!("Heavy exponent (100 ops): {:?}", duration_heavy);
    
    let diff = if duration_light > duration_heavy {
        duration_light.as_millis() - duration_heavy.as_millis()
    } else {
        duration_heavy.as_millis() - duration_light.as_millis()
    };
    
    let avg = (duration_light.as_millis() + duration_heavy.as_millis()) / 2;
    let percent_diff = (diff as f64 / avg as f64) * 100.0;
    
    println!("Różnica czasu: {:.2}%", percent_diff);
    
    assert!(percent_diff < 15.0, "Implementacja nie jest constant-time! Różnica > 15%");
}