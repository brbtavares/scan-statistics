mod map;
mod point;
mod region;
fn main() {
    let expect_thres_percent: f64 = 0.1;
    let (map, expectation_thres) = crate::map::create_map(expect_thres_percent);
    let cluster_candidates = crate::map::create_cluster_candidates(&map, expectation_thres);
    for cluster in cluster_candidates.iter() {
        println!("{:?}", cluster.len());
    }
}
