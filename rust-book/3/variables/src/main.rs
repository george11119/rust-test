fn main() {
    let x = 5;
    println!("x is {x}");
    // rust variables are immutable by default. the line below would cause an error.
    // x = 6;

    // to make a variable mutable, add the 'mut' keyword after 'let'

    let mut y = 5;
    println!("y is {y}");
    y = 7;
    println!("y is {y}");

    // constants cannot be changed. the difference between constants and variables is that variables
    // can be declared mutable, but constants cannot.
    //
    // use snake case and uppercase name to declare constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing:
    let z = 5;

    // replace the first z declaration with this one.
    let z = z + 1;

    {
        // replace the 2nd z declaration with this one.
        let z = z * 2;
        println!("z is {z} in the inner scope");
    }

    // scope ends, z goes back to being the 2nd z
    println!("z is {z}");
}
