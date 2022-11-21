use std::thread;

pub fn thread_sample() -> () {
    let handle = thread::spawn(|| println!("Hello world!"));
    dbg!(handle.join());
}
