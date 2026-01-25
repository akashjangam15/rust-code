#[derive(Debug)]
enum Diff_type{
    Intiger(i32),
    Float(f64),
    Text(String)
}

fn main(){
    let row=vec![Diff_type::Intiger(4),
    Diff_type::Float(5.6),
    Diff_type::Text(String::from("akash"
    ))];

    println!("{:?}",row);

}