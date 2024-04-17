mod distance;
mod map;
mod point;
mod region;
fn main() {
    let (map, expectation_thres) = crate::map::create_map();
    let cluster_candidates = crate::map::create_cluster_candidates(&map, expectation_thres);
}
