//iterators


//iterators using loops

use std::vec;

// fn main(){
//     let v1=vec![1,2,3];
//     // let v1_iter=v1.iter();
//     // println!("{:?}",v1_iter);

//     for v in v1.iter(){
//         print!("{} ",v);
//     }

//     print!("{:?}",v1);
// }


//iter_mut --- a mutable iterator
// fn main(){
//     let mut v1=vec![1,2,3,4];

//     let mut v1_iter=v1.iter_mut();

//     for val in v1_iter{
//         *val=*val+1;
//     }
// }

//iter.next
// fn main(){
//     let mut  num=vec![1,2,3];
//     let mut num_iter=num.iter_mut();

//     let first_num=num_iter.next();
//     let second_num=num_iter.next();
//     let third_num=num_iter.next();
//     let fourth_num=num_iter.next();

//     match first_num{
//         Some(val)=>println!("{}",val),
//         None=>println!("Nothing here"),
//     }
//       match fourth_num{
//         Some(val)=>println!("{}",val),
//         None=>println!("Nothing here"),
//     }


    // while let Some(val)=num_iter.next(){
    //     print!("{} ",val);
    // }
// }


fn main(){
    let mut nums=vec![1,2,3];
    let mut iter=nums.into_iter();

    for val in iter{
        println!("{}",val);
    }
    // print!("{:?}",nums);

}
 