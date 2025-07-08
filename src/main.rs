use std::fs::File;
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

    let  mut file=File::open(&input.trim())?;

    let mut file_contents=String::new();
    
    file.read_to_string(&mut file_contents)?;

    
    Ok(file_contents)
}
