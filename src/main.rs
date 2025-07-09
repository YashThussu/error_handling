use std::fs;
use std::io::{self,stdin};

mod option_error_prop;
mod project;

fn main() {
    
   let file_result=read_file();

   match file_result {

    Ok(content)=>println!("{content}"),
    Err(error)=>eprintln!("Error encountered:{error:?}")

   };

   option_error_prop::option_error();

   let write_result=project::write_to_file();

   match write_result {
      Ok(_)=>println!("Write Successful"),
      Err(error)=>eprintln!("Error Encountered: {error}")
   }
    

}

fn read_file()->Result<String,io::Error>{

     println!("Please enter tha path of the file:");
    let mut input=String::new();

    stdin().read_line(&mut input)?; // this terminates the code if it produces an error

    fs::read_to_string(input.trim())

    
}
