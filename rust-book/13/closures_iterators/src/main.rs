fn main() {
    // closures
    // let x = || -> u32 { 5 };
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;

    let x = add_one_v3(5);

    println!("x is {x}");

    let mut my_vec = vec![1, 2, 3];
    println!("Before push: {:?}", my_vec);

    let mut borrows_mutably = || my_vec.push(7);
    borrows_mutably();
    println!("After push: {:?}", my_vec);

    // iterators
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec1_iter = vec1.iter();

    println!("vec1: {:?}", vec1);
    for i in vec1_iter {
        println!("{i}");
    }

    let vec1_doubled: Vec<_> = vec1.iter().map(|x| x * 2).collect();
    let vec1_doubled_iter = vec1_doubled.iter();

    println!("vec1_doubled: {:?}", vec1_doubled);
    for i in vec1_doubled {
        println!("{i}");
    }

    let vec2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let even_vals_only: Vec<_> = vec2.iter().filter(|&x| x % 2 == 0).collect();
    println!("vec2: {:?}", vec2);
    println!("vec2 even vals only: {:?}", even_vals_only);

    let sum_of_even: u32 = vec2.iter().filter(|&x| x % 2 == 0).sum();
    println!("sum of even vals: {}", sum_of_even);
}
