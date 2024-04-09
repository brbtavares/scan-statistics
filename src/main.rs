mod definitions;
use crate::definitions::*;

fn main() {
    let (map, expectation_threshold) = create_map();
    let dist_matrix = create_dist_matrix(&map);
}
