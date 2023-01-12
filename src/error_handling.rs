use std::fs::File;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn _error_handling() {
    let result1 = File::open("resources/file1.txt");
    let result3 = File::open("resources/file3.txt");

    let file1 = match result1 {
        Ok(file) => file,
        Err(err) => panic!("File not found: {:?}", err)
    };

    let file3 = match result3 {
        Ok(file) => file,
        Err(err) => {
            println!("Error {:?}", err);
            match err.kind() {
                ErrorKind::NotFound => match File::create("resources/file3.txt") {
                    Ok(file) => file,
                    Err(err) => panic!("Error in creating file {:?}", err)
                },
                _ => panic!("Error in creating file")
            }
        }
    };
}