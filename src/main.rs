mod definitions;
use crate::definitions::*;
use compute::distributions::Uniform;
use compute::distributions::DiscreteUniform;

fn main() {
    let mut centroid_jitter = Uniform::new(0.0, 0.5);
    let mut expectation_gen = DiscreteUniform::new(0, 10);
    let r = Region::new(2, 4, Point::new(0f64,0f64), false);
    println!("The expectation of the region is {} and the x coordinate of its centroid is {}.", r.expectation, r.centroid.x);
}
