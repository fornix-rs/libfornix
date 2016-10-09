use rand::*;
use std::any::*;

/// Trait that allows an object to create random values
pub trait RandomNumberProvider {
    fn generate_array(&mut self, length: usize, range_min: f64, range_max: f64) -> Vec<f64>;
    fn generate_number(&mut self, range_min: f64, range_max: f64) -> f64;
}

pub struct OsRandomNumberProvider {
    os: os::OsRng,
}

impl OsRandomNumberProvider {
    pub fn new() -> Option<Self> {
        match os::OsRng::new() {
            Ok(os) => {
                Some(OsRandomNumberProvider {
                    os: os,
                })
            },
            Err(_) => {
                None
            }
        }
    }
}

impl RandomNumberProvider for OsRandomNumberProvider {
    fn generate_array(&mut self, length: usize, range_min: f64, range_max: f64) -> Vec<f64> {
        let mut vec = Vec::new();

        for _ in 0..length {
            vec.push(self.generate_number(range_min, range_max));
        }

        vec
    }

    fn generate_number(&mut self, range_min: f64, range_max: f64) -> f64 {
        self.os.gen_range::<f64>(range_min, range_max)
    }
}
