// src/utils/input.rs
use std::fs;
use anyhow::Result;

pub fn read_day_input(day: u8) -> Result<String> {
    let path = format!("input/day{:02}.txt", day);
    Ok(fs::read_to_string(path)?)
}