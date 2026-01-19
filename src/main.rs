use std::{collections::HashMap, vec};

// fn group_vlaues_by_keys(vec:Vec<(String,i32)>)->HashMap<String,i32>{
//     let mut hm=HashMap::new();
//     for (key,value) in vec{
//         hm.insert(key,value );
//     }
//     return hm;
// }


// fn main(){
//     let vec=vec![(String::from("Akash"),21),(String::from("Jyoti"),21)];
//     let hm=group_vlaues_by_keys(vec);
//     println!("{:?}",hm);
// }

fn group_vlaues_by_keys(vec:Vec<User>)->HashMap<String,i32>{
    let mut hm=HashMap::new();
    for user1 in vec{
        hm.insert(user1.name,user1.age );
    }
    return hm;
}

struct User{
    name:String,
    age:i32,
}

fn main(){
    let vec = vec![
        User {name:String::from("Akash"), age: 21 },
        User { name: String::from("Jyoti"), age: 21 },
    ];

    let hm = group_vlaues_by_keys(vec);

    println!("{:?}",hm);
}