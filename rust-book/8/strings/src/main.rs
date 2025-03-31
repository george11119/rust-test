fn main() {
    // let mut s = String::new();
    // let s = "hello world".to_string();
    // let s = String::from("initial contents");
    // let hello = String::from("你好");

    // s.push_str("Hello ");
    // s.push_str("world!");
    // println!("{}", s);

    let mut lol = String::from("lo");
    lol.push('l');
    println!("{}", lol);

    let s1 = String::from("hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // cant print s1, ownership of s1 string moved to s3
    // println!("{}", s1);
    // s2 is ok, as only reference was used to add to s3
    println!("{}", s2);
    println!("{}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s4 = format!("{tic}-{tac}-{toe}");
    println!("{tic} + {tac} + {toe} = {s4}");

    let hello = &s3[0..5];
    println!("{}", hello);

    let chars = "Зд你好haha怎么了asldjf";

    println!("chars = {}", chars);
    print!("chars: ");
    for c in chars.chars() {
        print!("{}, ", c);
    }
    println!();

    print!("bytes: ");
    for b in chars.bytes() {
        print!("{}, ", b);
    }
    println!();
}
