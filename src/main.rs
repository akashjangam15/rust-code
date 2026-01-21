//traits

pub trait Summary {
    fn summirize(&self){
        println!("hii i there from summ ");
    }
    
}
pub trait Fix {
    fn fix(&self){
        println!("hii i there from fix ");
    }
    
}

struct User{
    name:String,
    age:i32,
}

impl Summary for User{}
impl Fix for User{}

fn main(){
    let user=User{
        name:String::from("Akash"),
        age:21,
    };
    notify(user);

}

fn notify<T: Summary + Fix>(u:T){
    println!("{:?} {:?}",u.summirize(),u.fix());
}