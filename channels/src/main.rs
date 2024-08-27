use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx_1, rx) = mpsc::channel();

    let tx_2 = tx_1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Thread 1: sending message..");
            tx_1.send(val).unwrap();
            println!("Thread 1: i am going to sleep...");
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            println!("Thread 2: sending message..");
            tx_2.send(val).unwrap();
            println!("Thread 2: i am going to sleep...");
            thread::sleep(Duration::from_secs(2));
        }
    });

    let mut count = 0;
    for received in rx {
        println!("Main: i just received message: {received}");
        count += 1;
        if count == 2 {
            println!();
            count = 0;
        }
    }
}
