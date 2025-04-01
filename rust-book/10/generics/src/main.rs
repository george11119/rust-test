struct Point<T> {
    x: T,
    y: T,
}

fn largest_val<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];

    for val in list {
        if val > max {
            max = val;
        }
    }

    max
}

fn main() {
    let char_list = ['a', 'b', 'c', 'd'];
    println!("The max in {:?} is {}", char_list, largest_val(&char_list));

    let u64_list = [1, 2, 3, 4, 5];
    println!("The max in {:?} is {}", u64_list, largest_val(&u64_list));

    let int_point = Point { x: 5, y: 6 };
    let float_point = Point { x: 5.2, y: 6.1 };
}
