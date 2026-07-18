use std::time::{Duration, UNIX_EPOCH, SystemTime};
use std::thread::{sleep};

fn main() {
    println!("Start app on Rust!");
    
    let mut counter = 0;
    let maxCounter = 10;

    loop {
        sleep(Duration::from_secs(1));

        counter += 1;

        println!("Still running... {}", counter);

        if counter >= maxCounter {
            counter = 0;
            println!("big number!");
        } else {
            println!("little number!");
        }
    }
}