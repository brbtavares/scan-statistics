pub struct Region {
    pub(crate) expectation: i64,
    pub(crate) cases: i64,
    pub(crate) centroid: Point,
    pub(crate) is_cluster: bool,
}

impl Region {
    pub fn new(expectation: i64, cases: i64, centroid: Point, is_cluster: bool) -> Self {
        Region {
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