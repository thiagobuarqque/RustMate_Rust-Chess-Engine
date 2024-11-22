use std::{fs, fs::OpenOptions, io::Write};

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn append_to_file(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    // println!("Writing logs...");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true) // Creates the file if it does not exist
        .open(file_path)?;

    writeln!(file, "{}", content)?; // `writeln!` adds a newline after the content
    Ok(())
}
