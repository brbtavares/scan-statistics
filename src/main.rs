mod definitions;

use crate::definitions::*;


fn main() {
    let region = Region {
        index: 1,
        expectation: 1,
        cases: 1,
        centroid: Point {
            x: 0.0,
            y: 0.0,
        },
        is_cluster: false,
    };

    region.plot();
}
