use futures::{Future, StreamExt, stream};
use tokio::time::{Sleep, sleep};
use std::{task::Poll, time::Duration};

struct Num {
    number: usize,
    polled: bool,
    sleep: Sleep,
}

impl Num {
    fn new(number: usize) -> Self {
        Self {
            number,
            polled: false,
            sleep: sleep(Duration::from_millis(rand::random::<u8>() as u64)),
        }
    }
}

impl Future for Num {
    type Output = usize;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut this = unsafe{self.get_unchecked_mut()};
        if this.polled {
            let future_sleep = unsafe {std::pin::Pin::new_unchecked(&mut this.sleep)};
            match future_sleep.poll(cx) {
                std::task::Poll::Pending => {
                    println!("pending");
                    return std::task::Poll::Pending
                },
                std::task::Poll::Ready(_) => (),
            }
            println!("{} is ready", this.number);
            Poll::Ready(this.number)
        } else {
            cx.waker().wake_by_ref();
            println!("{} is pending", this.number);
            this.polled = true;
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    // let mut res = stream::iter(0..100)
    //     .map(|number| async move {
    //         sleep(Duration::from_millis(rand::random::<u8>() as u64)).await;
    //         println!("{}", number);
    //     }).buffer_unordered(5);
    // while let Some(()) = res.next().await {}

    let res = stream::iter(0..100)
        .map(Num::new).buffered(10)
        .for_each(|number| async move {
            println!("{}", number);
        });
    res.await
}
