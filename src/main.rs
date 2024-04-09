mod definitions;
use crate::definitions::*;

fn main() {
    let (map, _expectation_threshold) = create_map();
    let dist_matrix = create_dist_matrix(&map);
    println!("{:?}", dist_matrix);
}
