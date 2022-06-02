use rand::{thread_rng, Rng};

pub fn random_f64() -> f64 {
    thread_rng().gen::<f64>()
}

pub fn random_f64_min_max(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
