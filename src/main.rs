use std::string;

fn get_string_length(s: &str)->usize{
    return s.chars().count();
}

fn main(){
    let name=String::from("Akash");
    let len=get_string_length(&name);

    println!("length of the string: {:?}",len);
}

