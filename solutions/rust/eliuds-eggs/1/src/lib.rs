pub fn egg_count(display_value: u32) -> usize {
    let mut eggs:usize = 0;
for n in 0..32 {
    
    if (display_value >> n & 1) == 1 {
        eggs += 1;
    } 
    }
    eggs
}
