use crate::region::Region;

#[derive(Debug)]
pub struct Distance {
    region_idx: [usize; 2],
    d: f64,
}

impl Distance {
    pub fn new(r1: &Region, r2: &Region) -> Self {
        Distance {
            region_idx: [r1.idx, r2.idx],
            d: r1.dist(r2),
        }
    }
}

pub fn create_dist_vec(map: &[Region]) -> Vec<Distance> {
    let l = map.len();
    let mut m: Vec<Distance> = Vec::with_capacity(l * (l - 1) / 2);
    for i in 0..(l - 1) {
        for j in (i + 1)..l {
            m.push(Distance::new(&map[i], &map[j]));
        }
    }
    m
}
