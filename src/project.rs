use std::io::{self, stdin};
use std::fs;

pub fn write_to_file()->Result<(),io::Error>{

    let mut path=String::new();
    println!("Path of file to write into:");

    stdin().read_line(&mut path)?; // enter the path into path variable

    let mut contents=String::new();
    println!("Enter the contents you want to write into the file:");
    stdin().read_line(&mut contents)?;

    println!("file path : {}",&path);
    println!("Content : {}",&contents);

    fs::write(path.trim(), contents)?;
    Ok(()) // theres nothing to return hence an Ok here
}