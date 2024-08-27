use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            println!("Thread: sending message..");
            tx.send(val).unwrap();
            println!("Thread: i am going to sleep...");
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("Main: i just received message: {received}");
    }
}
