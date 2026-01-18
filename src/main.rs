// struct User{
//     active :bool,
//     username: String,
//     age:i32,
// }

// fn main(){
//     let user=User{
//         username: String::from("Akash"),
//         active:true,
//         age:21,

//     };
//     println!("{}",user.username);
// }

//implementing struct

struct rect{
    width: u32,
    height: u32,
}

impl rect{
    fn area(&self)->u32{
        return self.width*self.height;
    }
}

fn main(){
    let rect1=rect{
        width:10,
        height:20,
    };
    println!("{}",rect1.area());  
}