use std::thread;

fn main(){
    let handler=thread::spawn(|| {
        for i in 1..10{
            println!("hiii number {i} from spawned thread!");
        }
    });

    for i in 1..10{
        println!("hiii number {i} from main thread!");
    }
    handler.join();

}
