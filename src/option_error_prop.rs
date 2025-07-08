pub fn option_error(){
    
    let mut animals:Vec<&str>=vec!["Giraffe","Monkey","Zebra"];
    println!("{:?}",length_of_last_element(&mut animals));
    println!("{:?}",length_of_last_element(&mut animals));
    println!("{:?}",length_of_last_element(&mut animals));
    println!("{:?}",length_of_last_element(&mut animals));

}

fn length_of_last_element(input: &mut Vec<&str>)->Option<usize>{

    let last_elemnt=input.pop()?;

    Option::Some(last_elemnt.len())
}