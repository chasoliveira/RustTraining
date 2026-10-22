use std::sync::atomic::{AtomicU32, Ordering};

const ABSOLUTE_ZERO_C: f64 = -273.15;
static CONVERTION_COUNT: AtomicU32 = AtomicU32::new(0);

fn convert_c_to_f(c: f64) -> f64 {
    if c < ABSOLUTE_ZERO_C {
        return f64::NAN;
    }
    CONVERTION_COUNT.fetch_add(1, Ordering::Relaxed);
    c * 9.0 / 5.0 * 32.0
}
fn main() {
    println!("Exercise: Type Safe - Temperature!");
    let temp = "98.6";
    let temp: f64 = temp.parse().unwrap();
    let temp = convert_c_to_f(temp);

    println!("{temp:.1}°F");
    println!("Conversions: {}", CONVERTION_COUNT.load(Ordering::Relaxed));
}
