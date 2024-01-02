use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn read_file() {
    let file = File::open("./non_existent_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        },
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
}

fn write_to_file(content: &str) {
    let mut file = match File::create("./output.txt") {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied while creating file: {}", error)
                }
                _ => {
                    panic!("Error creating file: {}", error)
                }
            }
        }
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Successfully wrote to file 🎉🎉🎉🎉🎉🎉"),
        Err(error) => {
            panic!("Error writing to file: {}", error)
        }
    }
}

fn main() {
    read_file();
    
    let content_to_write = "This text will be written to the file.";
    write_to_file(content_to_write);
}
