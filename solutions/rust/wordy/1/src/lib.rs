pub fn answer(command: &str) -> Option<i32> {
    // Sanitize the command string to remove extra words. If the string contains
    // no digits, return None.

    let sanitized = command.strip_prefix("What is");
    let sanitized = if sanitized.is_some() {
        sanitized.unwrap().strip_suffix("?")
    } else {
        sanitized
    };
    if sanitized.is_none() {
        return None;
    }
    let command = sanitized
        .unwrap()
        .trim_start()
        .trim_end()
        .split("by")
        .collect::<String>();
    let command = command.split_whitespace().collect::<Vec<&str>>();

    if command.len() == 1 {
        return Some(command.get(0).unwrap().parse::<i32>().unwrap());
    }
    if command.is_empty() {
        return None;
    }

    // Items to be evaluated cannot be as follows "5 plus 5 plus or"
    // 5 plus 5 5
    if command.len() == 4 {
        return None;
    }

    let mut result: i32;
    // Check if the first item of to be evaluates is a number. If not
    // dont do anything.
    if command.get(0).unwrap().parse::<i32>().is_ok() {
        result = command.get(0).unwrap().parse::<i32>().unwrap();
    } else {
        return None;
    }
    for (i, item) in command.iter().enumerate() {
        if item.parse::<i32>().is_err() {
            let right_o = command.get(i + 1);
            if right_o.is_none() || right_o.unwrap().parse::<i32>().is_err() {
                return None;
            }
            match item {
                &"plus" => result += right_o.unwrap().parse::<i32>().unwrap(),
                &"minus" => result -= right_o.unwrap().parse::<i32>().unwrap(),
                &"multiplied" => result *= right_o.unwrap().parse::<i32>().unwrap(),
                &"divided" => result /= right_o.unwrap().parse::<i32>().unwrap(),
                _ => break,
            }
        }
    }
    Some(result)
}
