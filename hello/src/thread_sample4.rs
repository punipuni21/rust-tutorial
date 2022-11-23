use std::sync::mpsc;
use std::thread;

pub fn thread_sample4() -> () {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello world!");

    let _ = handle.join();
}
