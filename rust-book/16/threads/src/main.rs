use std::sync::{Mutex, mpsc, Arc};
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
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    println!("Its over");

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
