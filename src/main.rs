use std::{thread, time::Duration};
use rand::Rng;

fn main() {
    let mut thread_handles = Vec::new();
    for i in 1..100 {
        let delay = rand::thread_rng().gen_range(1500..4000);
        let builder = thread::Builder::new().name(format!("thread-{}", i));
        thread_handles.push(builder.spawn(move || {
            thread::sleep(Duration::from_millis(delay));
            println!("线程：{}, 延迟 {} ms", thread::current().name().unwrap(), delay);
        }).unwrap());
    }

    for h in thread_handles {
        h.join().unwrap();
    }
}
