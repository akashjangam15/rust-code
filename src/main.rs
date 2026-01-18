// use std::ptr::null;

// //option enum
// fn main(){
//     let index=find_first_a(String::from("Akash"));
//     match index{
//         Some(value)=>println!("index is {}",value),
//         None=>println!("a not found"),
//     }

// }

// fn find_first_a(s:String)->Option<i32> {

//     for (index,c) in s.chars().enumerate()
//     {

//         if c=='a'{
//             return Some(index as i32);
//         }
//     }
//     return None;
// }


use std::ptr::null;

enum CostumOption{
    Some(i32),
    None,
}

//option enum
fn main(){
    let index=find_first_a(String::from("Akash"));
    match index{
        CostumOption::Some(value)=>println!("index is {}",value),
        CostumOption::None=>println!("a not found"),
    }

}

fn find_first_a(s:String)->CostumOption {

    for (index,c) in s.chars().enumerate()
    {

        if c=='a'{
            return CostumOption::Some(index as i32);
        }
    }
    return CostumOption::None;
}
