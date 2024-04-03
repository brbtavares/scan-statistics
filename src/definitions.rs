pub struct Region {
    pub(crate) index: i64,
    pub(crate) expectation: i64,
    pub(crate) cases: i64,
    pub(crate) centroid: Point,
    pub(crate) is_cluster: bool,
}

pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
}