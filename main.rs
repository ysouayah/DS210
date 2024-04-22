use std::fs::File;
use std::io::{BufReader, BufRead, Result, Write};

fn read_data_file(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

fn clean_data(data: Vec<String>) -> Vec<String> {
    // Implement your data cleaning logic here
    // For demonstration, let's just return the data as is
    data
}

fn write_cleaned_data(filename: &str, cleaned_data: &[String]) -> Result<()> {
    let mut file = File::create(filename)?;
    for line in cleaned_data {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn main() {
    let filename = "/Users/ysfsouayah/Downloads/twitter_combined.txt";
    let cleaned_filename = "cleaned-twitter.txt";

    // Read the data file
    let data = match read_data_file(filename) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading data file: {}", err);
            return;
        }
    };

    // Clean the data
    let cleaned_data = clean_data(data);

    // Write the cleaned data to a new file
    if let Err(err) = write_cleaned_data(cleaned_filename, &cleaned_data) {
        eprintln!("Error writing cleaned data to file: {}", err);
    } else {
        println!("Cleaned data written to {}", cleaned_filename);
    }
}