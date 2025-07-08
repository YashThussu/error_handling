use std::fs;
use std::io::{self,stdin,Read};

fn main() {
    
   let file_result=read_file();

   match file_result {

    Ok(content)=>println!("{content}"),
    Err(error)=>eprintln!("Error encountered:{error:?}")

   };
    

}

fn read_file()->Result<String,io::Error>{

     println!("Please enter tha path of the file:");
    let mut input=String::new();

    stdin().read_line(&mut input)?; // this terminates the code if it produces an error

    fs::read_to_string(input.trim())
}
