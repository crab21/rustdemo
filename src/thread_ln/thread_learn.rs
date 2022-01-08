use std::{thread, time::Duration};

pub fn thread_test_spawn() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

use std::sync::mpsc::channel;

pub fn thread_test_chan() {
    let (tx, rx) = channel();

    let sender = thread::spawn(move || {
        tx.send("Hello, thread".to_owned())
            .expect("unable to send on channel");
    });

    let rece = thread::spawn(move || {
        rx.recv().expect("unable to receive from channel");
    });

    sender.join().expect("The sender thread has panicked");
    rece.join().expect("The receiver thread has panicked");
}
