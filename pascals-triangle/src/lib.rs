use std::cmp::min;
use std::iter;

pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v: Vec<Vec<u32>> = (0..min(2, self.row_count))
            .map(|x| vec![1; (x + 1) as usize])
            .collect();

        for _ in 2..self.row_count {
            let last = v.last().unwrap();
            let l_padding = iter::once(&0).chain(last.iter());
            let r_padding = last.iter().chain(iter::once(&0));
            let row = l_padding.zip(r_padding).map(|(i, j)| i + j).collect();
            v.push(row);
        }

        v
    }
}
