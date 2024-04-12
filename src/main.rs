mod map;
mod matrix;
mod point;
mod region;
fn main() {
    let (map, _expectation_threshold) = crate::map::create_map();
    let dist_matrix = crate::matrix::create_dist_matrix(&map);
    println!("{:?}", dist_matrix);
}
