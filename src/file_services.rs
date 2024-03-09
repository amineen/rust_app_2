use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

pub fn read_file_content(name: &str) -> String {
    let file_result = File::open(name);
    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                let mut file = match File::create(name) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                };
                match file.write_all(b"Hello World, new file created!") {
                    Ok(_) => (),
                    Err(e) => panic!("Problem writing to the file: {:?}", e),
                };
                file
            }
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn write_text_to_file(name: &str, text: &str) {
    //get file handle
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(name)
        .unwrap();

    file.write_all(text.as_bytes()).unwrap();
}

pub fn file_exists(name: &str) -> bool {
    let file_result = File::open(name);
    match file_result {
        Ok(_) => true,
        Err(_) => false,
    }
}
