use compute::distributions::*;
pub struct Region {
    pub(crate) idx: usize,
    pub(crate) expectation: f64,
    pub(crate) cases: f64,
    pub(crate) centroid: Point,
    pub(crate) is_cluster: bool,
}

impl Region {
    pub fn new(idx: usize, expectation: f64, cases: f64, centroid: Point, is_cluster: bool) -> Self {
        Region {
            idx,
            expectation,
            cases,
            centroid,
            is_cluster,
        }
    }    
}

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point {
            x,
            y,
        }
    }
    
}

pub fn create_map() -> Vec<Region> {
    let centroid_jitter = Uniform::new(-0.5, 0.5);
    let expectation_gen = Uniform::new(0., 10.);
    let mut expectation_gen_sample: f64;
    let mut v: Vec<Region> = Vec::new();
    let mut k: usize = 0;
    let cluster_indices: Vec<Vec<i32>> = vec![vec![5,7], vec![5,8], vec![6,6],vec![6,7], vec![6,8], vec![6,9], vec![7,5], vec![7,6], vec![7,7], vec![7,8], vec![7,9], vec![7,10], vec![8,6], vec![8,7], vec![8,8], vec![8,9], vec![9,7], vec![9,8]];

    for i in 1..14 {
        for j in 1..14 {
            expectation_gen_sample = expectation_gen.sample();
            v.push(Region::new(k, expectation_gen_sample, Poisson::new(expectation_gen_sample).sample(), Point::new(f64::from(i)+centroid_jitter.sample(),f64::from(j)+centroid_jitter.sample()), if cluster_indices.contains(&vec![i,j])  {true} else {false}));
            k = k + 1;
        }
    }

    v
}