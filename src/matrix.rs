#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
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

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> bool {
        if row < self.rows && col < self.cols {
            self.data[row][col] = value;
            true
        } else {
            false
        }
    }
}

pub fn create_dist_matrix(map: &Vec<crate::region::Region>) -> Matrix {
    let l = map.len();
    let mut m = Matrix::new(l, l);
    for i in 0..l {
        for j in i..l {
            m.set(i, j, map[i].dist(&map[j]));
        }
    }
    m
}
