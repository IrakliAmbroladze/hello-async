use std::time::Duration;
fn main() {
    trpl::run(async {
        println!("First sentence");
        trpl::sleep(Duration::from_millis(3000)).await;
        println!("Second sentence after 3 seconds");
    });
}
