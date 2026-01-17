fn main() {
    println!("{}",fib(4));
}

fn fib(num:i32)->i64{

    let mut first=0;
    let mut second=1;

    if num==0{
        return first;
    }
    if num==1{
        return 1;
    }
    for i in 1..num-1{
        let temp=second;
        second = second + first;
        first= temp;
    }
    return second;

}
