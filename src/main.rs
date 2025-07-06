use std::fs::File;
use std::process;
use std::io::{stdin,Read};

fn main() {
    
    println!("Please enter tha path of the file:");
    let mut input=String::new();

    let file_path_result=stdin().read_line(&mut input);

    if let Err(error)= file_path_result{
        eprintln!("Somthing went wrong the error msg is {error:?}");
        process::exit(1);
    };

    let  file=File::open(&input.trim());
    let mut result=match file{
        Ok(success_file)=>success_file,
        Err(error)=>{
            eprintln!("Something went wrong reading the file:{error:?}");
            process::exit(1);
        }
    }; // need to make this mutable for read trait to work

    let mut file_contents=String::new();
    let read_operation=result.read_to_string(&mut file_contents); // read to string becomes available only after Read trait is brought in

    if let Err(error)= read_operation{
        eprintln!("Someething went wrong reading the string The error was {error:?}");
        process::exit(1);
    }
    
    println!("Result is :{result:#?}");

    println!("File Content: {file_contents}")
    

}
