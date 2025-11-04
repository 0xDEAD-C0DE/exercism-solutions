pub fn series(digits: &str, len: usize) -> Vec<String> {
    let length = digits.len();
    let mut i = 0;
    let mut step = len;
    let mut series: Vec<String> = vec![]; 
    if step > length {
        return series;
    }

    while i <= length {
        series.push(digits[i..step].to_string());
        i += 1;
        step += 1;
        if step > length {
            break;
        }
    }
    series
}

