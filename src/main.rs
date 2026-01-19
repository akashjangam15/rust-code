use chrono::{Local,Utc};

fn main(){
    let now=Utc::now();
    println!("{}",now);
}