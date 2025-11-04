use std::fs;

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug)]
pub struct Flags {
    number_lines: bool,
    file_names: bool,
    ignore_case: bool,
    whole_line: bool,
    invert_match: bool,
}
//#[derive(Debug)]
//pub struct Flags<'a> {
//pub struct Flags {
//    flags: Vec<String>,
//}
impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Self {
            number_lines: flags.contains(&"-n"),
            file_names: flags.contains(&"-l"),
            ignore_case: flags.contains(&"-i"),
            whole_line: flags.contains(&"-x"),
            invert_match: flags.contains(&"-v"),
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut contents: String;
    let mut result: Vec<String> = vec![];
    let numfiles = files.len();
    let pattern = if flags.ignore_case {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    for filename in files {
        contents = fs::read_to_string(filename)?;

        for (linenum, line) in contents.lines().enumerate() {
            let new_line = if flags.ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if ((!flags.whole_line && new_line.contains(&pattern)) || pattern == new_line)
                ^ flags.invert_match
            {
                let fmt_line = format!(
                    "{}{}{}",
                    if numfiles > 1 && !flags.file_names {
                        filename.to_string() + &":"
                    } else if flags.file_names {
                        filename.to_string()
                    } else {
                        "".to_string()
                    },
                    if flags.number_lines && !flags.file_names {
                        (linenum + 1).to_string() + &":"
                    } else {
                        "".to_string()
                    },
                    if flags.file_names {
                        "".to_string()
                    } else {
                        line.to_string()
                    }
                );
                if !result.contains(&filename.to_string()) {
                    result.push(fmt_line);
                }
            }
        }
    }
    Ok(result)
}
