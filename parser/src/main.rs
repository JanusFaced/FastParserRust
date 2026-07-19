use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let timeSleep = 5;
    let mut counter = 0;
    loop {
        let a = 100;
        let b = 10.0;
        let c = "100";
        let d = "10.0";
        let e = "Hello!";
        let f: [i64; 0] = [];
        let g: [f64; 0] = [];
        let h: [String; 0] = [];
        let i = true;

        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", d);
        println!("{:?}", e);
        println!("{:?}", f);
        println!("{:?}", g);
        println!("{:?}", h);
        println!("{:?}", i);

        thread::sleep(Duration::from_secs(timeSleep));
        counter += 1;
        println!("counter = {:?}", counter);
    }

    Ok(())
}
