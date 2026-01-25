use std::{collections::HashMap, string};

fn main(){

    // let mut scores=HashMap::new();
    // scores.insert(String::from("blue"),10);
    // scores.insert(String::from("yellow"), 20);
    // let team_name=String::from("blue");

    // let score=scores.get(&team_name).copied().unwrap_or(0);
    // println!("{}",score);

    let text="Hii I am Akash and I am a rust Developer.";
    let mut map=HashMap::new();

    for word in text.split_whitespace(){
        let count=map.entry(word).or_insert(0);
        *count+=1;
    }

    println!("{map:?}");


}