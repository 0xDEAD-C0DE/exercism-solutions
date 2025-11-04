pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut top: usize = 0;
    let mut bottom: usize = size as usize;
    let mut left: usize = 0;
    let mut right: usize = size as usize;
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; size as usize]; size as usize];
    let mut counter = 1;
    if size == 0 {
        return vec![];
    }
    while top <= bottom && left <= right {
        for i in left..right {
            matrix[top][i] = counter;
            counter += 1;
        }

        top += 1;
        for i in top..bottom {
            matrix[i][right.saturating_sub(1)] = counter;
            counter += 1;
        }
        right -= 1;

        if top <= bottom {
            for i in (left..right).rev() {
                matrix[bottom.saturating_sub(1)][i] = counter;
                counter += 1;
            }
            bottom -= 1;
        }
        if left <= right {
            for i in (top..bottom).rev() {
                matrix[i][left] = counter;
                counter += 1;
            }
            left += 1;
        }
    }
    matrix
}
