pub fn build_proverb(list: &[&str]) -> String {
 
    let mut proverb = String::new();
    if list.len() == 1 { 
        proverb.push_str(&format!("And all for the want of a {}.", list[0]));
        return proverb;
    }
    
        for (i, w) in list.iter().enumerate() {
            if i > list.len() - 2 {
                proverb.push_str(&format!("And all for the want of a {}.", list[0]));
                break;
            }
            proverb.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                w,
                list[i + 1]
            ));
        }
    proverb
}
