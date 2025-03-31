use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{fs, io};

// fn read_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result {
//         Ok(f) => f,
//         Err(e) => return Err(e),
//     };
//
//     let mut username = String::new();
//
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// fn read_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_file() -> Result<String, io::Error> {
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

fn read_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // panic!("seg fault core dumped ahah");
    // let v = vec![1, 2, 3];
    // v[99];
    //
    // let open_file_result = File::open("hello.txt");
    // let open_file = match open_file_result {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Could not create file: {e:?}"),
    //         },
    //         other_error => panic!("oh no"),
    //     },
    // };

    // let file = File::open("hello.txt").expect("hello.txt should be included");
    read_file();
}
