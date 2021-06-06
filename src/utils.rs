use std::io::{Write, stdin, stdout};

use crate::errors::CarajilloError;

pub fn get_user_input() -> Result<String, CarajilloError> {
    let mut out = String::new();
    println!("Select which definitions you want. For example: 1 3 7");
    let _ = stdout().flush();

    stdin().read_line(&mut out)?;

    Ok(out.strip_suffix("\n")
        .or(out.strip_suffix("\r\n"))
        .unwrap_or(&out).to_string())
}

pub fn parse_user_input(s: String) -> Vec<i32> {
    let mut ret: Vec<i32> = s
        .split(' ')
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    ret.dedup();

    ret
}
