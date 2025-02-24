use std::fs::{File, OpenOptions};
use std::io::Write;
use std::{
    fs::{self},
    io::Error,
};

type Result<T> = std::result::Result<T, Error>;
pub fn get_history() -> Result<String> {
    let history = match fs::read_to_string("history.txt") {
        Ok(h) => h,
        Err(e) => {
            // Create the file
            if let Err(e) = File::create("history.txt") {
                println!("Failed to create the file: {}", e);
                return Err(e);
            }
            String::new()
        }
    };
    Ok(history)
}

pub fn get_total() -> Result<f64> {
    let history = get_history().unwrap_or_else(|_| String::new());

    let values = history
        .lines()
        .map(|l| l.trim().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();

    let total: f64 = values.iter().sum::<f64>();

    let mut file = OpenOptions::new()
        .write(true) // Open the file for writing
        .truncate(true) // Erase the file's content if it exists
        .create(true)
        .open("history.txt")?;

    file.write_all(format!("{}\n", total).as_bytes())?;
    Ok(total)
}

pub fn add_value(value: f64) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true) // Open the file in append mode
        .create(true) // Create the file if it doesn’t exist
        .open("history.txt")?;

    file.write_all(format!("{}\n", value).as_bytes())?;

    Ok(())
}

pub fn remove_last_value() -> Result<f64> {
    let file_path = "history.txt";

    let content = fs::read_to_string(file_path)?;

    let mut val = 0.0;
    let mut lines: Vec<&str> = content.lines().collect();
    if !lines.is_empty() {
        val = lines.pop().unwrap().parse::<f64>().unwrap_or(0.0);
    }

    let new_content = lines.join("\n");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(new_content.as_bytes())?;

    let _ = get_total();
    Ok(val)
}
