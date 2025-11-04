
pub fn get_diamond(c: char) -> Vec<String> {
    let width: u8 = (c as u8 - 'A' as u8) * 2;
    let height = width / 2;
    let mut grid: Vec<String> = vec![];
    let mut s = String::new();
    // ····A····
    // ···B·B···
    // ··C···C··
    // ·D·····D·
    // E·······E
    // ·D·····D·
    // ··C···C··
    // ···B·B···
    // ····A····
    for row in 0..=height {
        for col in 0..=width {
            if col == (width / 2) - row {
                s.push((c as u8 - col) as char);
            } else if col == (width / 2) + row && row > 0 {
                s.push(('A' as u8 + row) as char);
            } else {
                s.push(' ');
            }
        }
        grid.push(s);
        s = String::new();
    }
    for row in 1..=height {
        for col in 0..=width {
            if col == row {
                s.push((c as u8 - row) as char);
            } else if col == width - row {
                s.push((c as u8 - row) as char);
            } else {
                s.push(' ');
            }
        }
        grid.push(s);
        s = String::new();
    }
    grid
    
}