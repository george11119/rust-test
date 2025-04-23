use std::pin::{Pin, pin};
use std::time::Duration;

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(val) = rx.recv().await {
                println!("Got: {val}");
            }
        });

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut, tx1_fut, rx_fut];
        trpl::join_all(futures).await;
    });
}
