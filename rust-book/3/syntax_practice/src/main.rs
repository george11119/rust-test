fn temp_conversion(temp_value: f32, temp_unit: &str) {
    if temp_unit == "fahrenheit" {
        let temp_value_celsius = (temp_value - 32.0) * (5.0 / 9.0);
        println!(
            "{temp_value:.2} degrees fahrenheit to celsius is {temp_value_celsius:.2} degrees celsius"
        );
    } else if temp_unit == "celsius" {
        let temp_value_fahrenheit = (temp_value * (9.0 / 5.0)) + 32.0;
        println!(
            "{temp_value:.2} degrees celsius to fahrenheit is {temp_value_fahrenheit:.2} degrees fahrenheit"
        );
    } else {
        println!("Invalid temperature unit!");
    }
}

fn nth_fibonacci_number(n: u32) -> u32 {
    let mut num_1: u32 = 0;
    let mut num_2: u32 = 1;

    if n == 0 { return num_1; }
    if n == 1 { return num_2; }

    let mut result: u32 = 0;
    let mut count: u32 = 1;

    while count != n {
        result = num_1 + num_2;
        num_1 = num_2;
        num_2 = result;
        count += 1;
    }

    result
}

fn main() {
    temp_conversion(100.0, "fahrenheit");
    temp_conversion(100.0, "celsius");

    for i in 1..10 {
        println!("{i} = {}", nth_fibonacci_number(i));
    }
}
