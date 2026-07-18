fn main() {
    println!("Hello, World from Rust!");
    println!("Press Ctrl+C to stop...");
    
    // Бесконечный цикл, чтобы программа не завершалась
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        println!("Still running... {}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
    }
}