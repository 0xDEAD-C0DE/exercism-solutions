#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    //table[i,w] = max(table[i-1,i], table[i-1, w - w[i]] + p[i])
    let mut table: Vec<Vec<usize>> = vec![vec![0; (max_weight + 1) as usize]; items.len() + 1];
    for i in 1..=items.len() {
        let weight = items[i - 1].weight as usize;
        let value = items[i - 1].value as usize;
        for w in 0..=(max_weight as usize) {
            if weight <= w {
                table[i][w as usize] = std::cmp::max(
                    table[i - 1][w],
                    table[i - 1][w.saturating_sub(weight)] + value,
                );
            } else {
                table[i][w] = table[i - 1][w];
            }
        }
    }
    println!("{:?}", table);
    table[items.len()][max_weight as usize] as u32
    // todo!("calculate the maximum value achievable with the given {items:?} and {max_weight}");
}
