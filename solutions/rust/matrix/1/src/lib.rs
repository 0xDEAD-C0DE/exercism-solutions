pub struct Matrix {
    grid: Vec<Vec<u32>>,
    // Implement your Matrix struct
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|x| {
                    x.split_whitespace()
                        .filter_map(|x| x.parse::<u32>().ok())
                        .collect()
                })
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        Some(self.grid.get(row_no.saturating_sub(1))?.to_vec())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if self.grid.get(0).unwrap().len() <= col_no.saturating_sub(1) {
            return None;
        }

        Some(
            self.grid
                .iter()
                .map(|x| *x.get(col_no.saturating_sub(1)).unwrap())
                .collect::<Vec<u32>>(),
        )
    }
}