use std::fs;
use std::io::{self,stdin};

mod option_error_prop;
mod project;

fn main() {
    
   // let file_result=read_file();

   // match file_result {

   //  Ok(content)=>println!("{content}"),
   //  Err(error)=>eprintln!("Error encountered:{error:?}")

   // };

<<<<<<< HEAD
   option_error_prop::option_error();
=======
   // option_error_prop::option_error();
>>>>>>> 590a1b7feb7c70b8673a3b2260b49633c81c1e70

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
