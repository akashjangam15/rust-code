//traits

pub trait Summary {
    fn summirize(&self)->String{
        return String::from("Summirized");
    }
    
}

struct User{
    name:String,
    age:i32,
}

impl Summary for User{
    fn summirize(&self)->String {
        return format!("The name is {} and age is {}",self.name,self.age);
    }
}

fn main(){
    let user=User{
        name:String::from("Akash"),
        age:21,
    };
    println!("{}",user.summirize());
}