fn main(){
    let mut s1=String::from("Akash");
    do_someting(&s1);
    println!("{}",s1);

}
fn do_someting(s2:& String){
    println!("{}",s2);
}
