enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn print_ip_addr(addr: IpAddrKind) {
    match addr {
        IpAddrKind::V4(a, b, c, d) => {
            println!("{}.{}.{}.{}", a, b, c, d);
        }
        IpAddrKind::V6(v6_addr) => {
            println!("{}", v6_addr);
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn let_else_test(foo: Option<u8>) {
    let Some(val) = foo else {
        println!("foo is None");
        return;
    };
    println!("foo is {val}");
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    print_ip_addr(home);

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let config_max: Option<u8> = Some(3);
    if let Some(x) = config_max {
        println!("The maximum is {x}")
    }

    let x = Some(5);
    let y: Option<u8> = None;
    let_else_test(x);
    let_else_test(y);
}
