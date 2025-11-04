pub fn is_armstrong_number(num: u32) -> bool {
    let mut temp = num;
    let mut n = num;
    let mut digit;
    let mut r;
    let mut sum:u32 = 0;
    let mut len = 0;
    while n != 0 {
        n = n / 10;
        len+= 1;
    }
    while temp != 0 {
    digit = temp / 10;
    r = temp % 10;
    temp = digit;
    sum = sum.saturating_add(r.pow(len));
    }
    sum == num
}