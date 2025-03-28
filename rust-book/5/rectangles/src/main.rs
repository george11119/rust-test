#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(renctangle: &Rectangle) -> u32 {
    renctangle.width * renctangle.height
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {}",
        area(&rect)
    );

    dbg!(&rect);
}