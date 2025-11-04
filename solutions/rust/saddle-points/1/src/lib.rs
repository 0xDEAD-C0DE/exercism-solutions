pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut coord: Vec<(usize, usize)> = vec![];
    let mut maximum: Vec<u64> = vec![];

    for (index, rows) in input.iter().enumerate() {
        if input[0].is_empty() {
            break;
        }
        // Find the maximum of a row.
        let mut max = input[index][0];
        for (i, row) in rows.iter().enumerate() {
            if max < *row {
                max = rows[i];
            }
        }
        maximum.push(max);
    }
    for (i, _) in input[0].iter().enumerate() {
        let mut min = input[0][i];
        // Find the minimun of a column
        //
        for (j, _) in input.iter().enumerate() {
            if min > input[j][i] {
                min = input[j][i];
            }
        }

        for (j, _) in input.iter().enumerate() {
            // If the minumun is equal to an element in  a column,
            // and it is the maximum of a row. Then it is a saddle point.
            if min == input[j][i] && min == maximum[j] {
                coord.push((j, i));
            }
        }
    }
    coord
}
