use std::fs::File;
use std::process;
use std::io::stdin;

fn main() {
    
    println!("Please enter tha path of the file:");
    let mut input=String::new();

    let file_path_result=stdin().read_line(&mut input);

    if let Err(error)= file_path_result{
        eprintln!("Somthing went wrong the error msg is {error:?}");
        process::exit(1);
    };

    let file=File::open(&input.trim());
    let result=match file{
        Ok(success_file)=>success_file,
        Err(error)=>{
            eprintln!("Something went wrong reading the file:{error:?}");
            process::exit(1);
        }
    };

    println!("Result is :{result:#?}");
    
}
