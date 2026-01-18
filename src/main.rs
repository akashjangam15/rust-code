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

// struct rect{
//     width: u32,
//     height: u32,
// }

// impl rect{
//     fn area(&self)->u32{
//         return self.width*self.height;
//     }
// }

// fn main(){
//     let rect1=rect{
//         width:10,
//         height:20,
//     };
//     println!("{}",rect1.area()); 

// }

enum Shape{
    Circle(f64),
    Rectangle(f64,f64),
    Square(f64),
}

fn calculate_area(shape:Shape)->f64{
    let area=match shape{
        Shape:: Square(a)=>a*a,
        Shape::Rectangle(x,b )=>x*b,
        Shape::Circle(z)=>3.14*z*z,
    };
    return area;
}

fn main(){
    let circle=Shape::Circle(5.0);
    let square= Shape::Square(4.0);
    let rectangel=Shape::Rectangle(3.0,6.0);

    let ans=calculate_area(rectangel);
    
    println!("{}",ans);

}
