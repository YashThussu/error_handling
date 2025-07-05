use std::fs::File;
use std::process;

fn main() {
    
    let file=File::open("C:\\Users\\YASH\\Desktop\\coding\\Rust\\story.txt");
    let result=match file{
        Ok(success_file)=>success_file,
        Err(error)=>{
            eprintln!("Something went wrong reading the file:{error:?}");
            process::exit(1);
        }
    };

    println!("Result is :{result:#?}");
    
}
