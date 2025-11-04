pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut pascal: Vec<Vec<u32>> = vec![];
        // for _row in 0..self.0 as usize {
        // pascal.push(vec![1]);
        // }

        for row in 0..self.0 as usize {
            pascal.push(vec![1]);
            for col in 1..=row {
                pascal
                    .get_mut(row)
                    .unwrap()
                    .push(binomial_coefficient(row as u32, col as u32));
            }
        }
        pascal
    }
}

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    let mut answer = 1;
    // let k = if n - k > k { n - k } else { k };
    for i in 0..k {
        answer *= n - i;
        answer /= i + 1;
    }
    answer
}
// fn factorial(num: u32) -> u32 {
//     (1..=num).product()
// }
