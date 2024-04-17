#[derive(Clone)]
pub struct Region {
    pub(crate) idx: usize,
    pub(crate) expectation: f64,
    pub(crate) cases: f64,
    pub(crate) centroid: crate::point::Point,
    pub(crate) is_cluster: bool,
}

impl Region {
    pub fn new(
        idx: usize,
        expectation: f64,
        cases: f64,
        centroid: crate::point::Point,
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
