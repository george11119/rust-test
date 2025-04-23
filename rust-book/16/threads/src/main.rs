use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Heres a vector: {:?}", v);
    });

    handle2.join().unwrap();

    // let (tx, rx) = mpsc::channel();
    //
    // thread::spawn(move || {
    //     let val = String::from("Created inside thread");
    //     tx.send(val).unwrap();
    // });
    //
    // let recieved = rx.recv().unwrap();
    // println!("{recieved}");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    println!("Its over");
}
