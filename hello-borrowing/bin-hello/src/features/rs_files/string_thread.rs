pub const STRING_THREAD_OK: &str = r#"fn main() {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let instance = String::from("hi");
        println!("Got from tx: {}", instance);
        
        tx.send(instance).unwrap();
        
    });

    let received = rx.recv().unwrap();
    println!("Got from rx: {}", received);
}
"#;

pub const STRING_THREAD_ERR: &str = r#"fn main() {
    use std::thread;
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let instance = String::from("hi");
        println!("Got from tx: {}", instance);
        
        tx.send(instance).unwrap();
        
        println!("val is {}", instance);
    });

    let received = rx.recv().unwrap();
    println!("Got from rx: {}", received);
}
"#;
