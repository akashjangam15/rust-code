fn main(){
    let  v1=vec![1,2,3];
    let mut v1_iter=v1.iter(); 
    let v1_iter_x=v1_iter.clone();

    // let total:i32=v1_iter.sum();  //consuming adeptor of vi_iter
    // println!("{}",total);
    // println!("{:?}",v1);


    let v1_iter2=v1_iter.map(|x| x+1);
    let v1_iter3=v1_iter_x.filter(|x|*x%2==0);
    for n in v1_iter2{
        println!("{}",n);
    }
    for n in v1_iter3{
        println!("{}",n);
    }

}