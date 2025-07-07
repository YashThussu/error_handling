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

    let file_path_result=stdin().read_line(&mut input);

    if let Err(error)= file_path_result{
        return Result::Err(error); // return the error here 
    };

    let  file=File::open(&input.trim());
    let mut result=match file{
        Ok(success_file)=>success_file,
        Err(error)=>{
            return Err(error);
        }
    }; // need to make this mutable for read trait to work

    let mut file_contents=String::new();
    let read_operation=result.read_to_string(&mut file_contents); // read to string becomes available only after Read trait is brought in

    if let Err(error)= read_operation{
        return Err(error);
    }
    
    Ok(file_contents)
}
