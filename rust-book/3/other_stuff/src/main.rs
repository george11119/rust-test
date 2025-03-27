fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", x[0]);
    println!("{}", plus_one(5));

    for element in x {
        println!("{element}")
    }

    if x[0] > 5 {
        println!("x[0] is greater than 5");
    } else if x[0] < 5 {
        println!("x[0] is less than 5");
    } else {
        println!("x[0] is equal to 5");
    }

    let condition = true;
    let y = if condition { 20 } else { 0 };
    println!("y is {y}");

    // infinite loop
    // loop {
    //     println!("???");
    // }

    let mut i = 0;
    while i != 10 {
        println!("{i}");
        i += 1;
    }

    for j in (1..10) {
        println!("{j}");
    }
}
