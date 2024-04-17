use crate::region::Region;
use compute::distributions::{Distribution, Poisson, Uniform};

pub fn create_map(expect_thres_percent: f64) -> (Vec<Region>, f64) {
    let centroid_jitter = Uniform::new(-0.5, 0.5);
    let expectation_gen = Uniform::new(0., 10.);
    let mut expectation_gen_sample: f64;
    let mut expectation_total: f64 = 0.;
    let mut v: Vec<Region> = Vec::new();
    let mut k: usize = 0;
    let cluster_indices: Vec<Vec<i32>> = vec![
        vec![5, 7],
        vec![5, 8],
        vec![6, 6],
        vec![6, 7],
        vec![6, 8],
        vec![6, 9],
        vec![7, 5],
        vec![7, 6],
        vec![7, 7],
        vec![7, 8],
        vec![7, 9],
        vec![7, 10],
        vec![8, 6],
        vec![8, 7],
        vec![8, 8],
        vec![8, 9],
        vec![9, 7],
        vec![9, 8],
    ];

    for i in 1..14 {
        for j in 1..14 {
            expectation_gen_sample = expectation_gen.sample();
            expectation_total += expectation_gen_sample;
            v.push(Region::new(
                k,
                expectation_gen_sample,
                Poisson::new(expectation_gen_sample).sample(),
                crate::point::Point::new(
                    f64::from(i) + centroid_jitter.sample(),
                    f64::from(j) + centroid_jitter.sample(),
                ),
                cluster_indices.contains(&vec![i, j]),
            ));
            k += 1;
        }
    }

    (v, expectation_total * expect_thres_percent)
}

pub fn create_cluster_candidates(m: &[Region], thres: f64) -> Vec<Vec<Region>> {
    let mut cluster_vec: Vec<Vec<Region>> = Vec::new();
    let l = m.len();
    for i in 0..l {
        let cluster: Vec<Region> = m
            .iter()
            .filter(|&c| c.dist(&m[i]) < thres)
            .cloned()
            .collect();
        cluster_vec.push(cluster);
    }
    cluster_vec
}
