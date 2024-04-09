use compute::distributions::*;

pub struct Region {
    pub(crate) idx: usize,
    pub(crate) expectation: f64,
    pub(crate) cases: f64,
    pub(crate) centroid: Point,
    pub(crate) is_cluster: bool,
}

impl Region {
    pub fn new(
        idx: usize,
        expectation: f64,
        cases: f64,
        centroid: Point,
        is_cluster: bool,
    ) -> Self {
        Region {
            idx,
            expectation,
            cases,
            centroid,
            is_cluster,
        }
    }

    pub fn dist(&self, r: &Region) -> f64 {
        ((self.centroid.x - r.centroid.x).powi(2) + (self.centroid.y - r.centroid.y).powi(2)).sqrt()
    }
}

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    fn set(&mut self, row: usize, col: usize, value: f64) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            true
        } else {
            false
        }
    }
}

pub fn create_map() -> (Vec<Region>, f64) {
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
                Point::new(
                    f64::from(i) + centroid_jitter.sample(),
                    f64::from(j) + centroid_jitter.sample(),
                ),
                cluster_indices.contains(&vec![i, j]),
            ));
            k += 1;
        }
    }

    (v, expectation_total * 0.15)
}

pub fn create_dist_matrix(map: &[Region]) -> Matrix {
    let l = map.len();
    let mut m = Matrix::new(l, l);
    for i in 0..l {
        for j in i..l {
            m.set(i, j, map[i].dist(&map[j]));
        }
    }
    m
}
