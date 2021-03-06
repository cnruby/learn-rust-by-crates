#[cfg(feature = "ok")]
fn main() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let instance = String::from("hi");
        println!("Got from tx: {}", instance);

        // the variable `instance` begin to move
        tx.send(instance).unwrap();
        // the variable `instance` moved here

        // ERROR: The variable `instance` borrowed here after move
        //println!("val is {}", instance);
    });

    let received = rx.recv().unwrap();
    println!("Got from rx: {}", received);
}

#[cfg(feature = "err")]
fn main() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let instance = String::from("hi");
        println!("Got from tx: {}", instance);

        // the variable `instance` begin to move
        tx.send(instance).unwrap();
        // the variable `instance` moved here

        // ERROR: The variable `instance` borrowed here after move
        println!("val is {}", instance);
    });

    let received = rx.recv().unwrap();
    println!("Got from rx: {}", received);
}

#[cfg(all(not(feature = "ok"), not(feature = "err")))]
fn main() {
    use aide::hello;
    hello();
}
