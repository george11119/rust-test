use std::time::Duration;

const SLEEP_TIME: u64 = 1;

fn main() {
    trpl::run(async {
        let fut_1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(SLEEP_TIME)).await;
            }
        };

        let fut_2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task");
                trpl::sleep(Duration::from_millis(SLEEP_TIME)).await;
            }
        };

        trpl::join(fut_1, fut_2).await;

        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("Hello"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("Got: {value}");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
