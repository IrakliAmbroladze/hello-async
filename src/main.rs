use std::time::Duration;

use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let messages = get_messages().timeout(Duration::from_millis(200));
        let intervals = get_intervals();
        let merged = messages.merge(intervals);
        while let Some(result) = messages.next().await {
            match result {
                Ok(message) => println!("Message is: {message}"),
                Err(reason) => eprintln!("Problem: {reason:?}"),
            }
        }
    });
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;
            tx.send(format!("Message: '{message}'")).unwrap();
        }
    });
    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = i32> {
    let (tx, rx) = trpl::channel();
    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });
    ReceiverStream::new(rx)
}
