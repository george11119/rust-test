enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![2, 4, 6];
    v.push(8);
    println!("vector: {v:?}");
    println!("vector size: {}", v.capacity());

    let mut third: &i32 = &v[2];
    println!("third element of vector is: {}", third);

    let fifth: Option<&i32> = v.get(4);
    match fifth {
        Some(val) => println!("fifth element of vector is {}", val),
        None => println!("no fifth value of vector"),
    }

    v.push(10);

    print!("vector: ");
    for i in &v {
        print!("{i} ")
    }
    println!();

    for i in &mut v {
        *i *= 2;
    }

    print!("vector: ");
    for i in &v {
        print!("{i} ")
    }
    println!();

    while let Some(top) = v.pop() {
        println!("{top}");
    }

    let row = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(1.1),
        SpreadsheetCell::Text(String::from("???")),
    ];
}
