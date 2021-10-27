use tokio::runtime;
use std::future::ready;

fn main() {
    let runtime = runtime::Builder::new_multi_thread().build().unwrap();

    for number in 1..100 {
        let future = async move {
            println!("number: {}", number);
        };
        runtime.spawn(future);
    }
    println!("ok");
    runtime.spawn(async move {
        println!("ok again");
    });

    let future = ready(3);
    let join_handle = runtime.spawn(future.clone());
    runtime.spawn(async move {
        println!("{}", join_handle.await.unwrap());
    });
}
