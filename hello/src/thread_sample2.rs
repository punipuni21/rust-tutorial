use std::thread;

pub fn thread_sample2() -> () {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("Hello, world!: {}", x);
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }
}
