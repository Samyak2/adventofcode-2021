use std::{
    error::Error,
    fs::{File, read_to_string},
    io::{BufRead, BufReader},
};

pub fn read_lines<F>(filename: &str, mut processor: F) -> Result<(), Box<dyn Error>>
where
    F: FnMut(String),
{
    let file = File::open(filename)?;

    BufReader::new(file).lines().for_each(|line| {
        if let Ok(line) = line {
            processor(line)
        }
    });
    Ok(())
}

pub fn read_string(filename: &str) -> String {
    read_to_string(filename).unwrap()
}
