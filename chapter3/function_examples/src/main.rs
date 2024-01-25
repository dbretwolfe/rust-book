fn main() {
    println!("Hello, world!");

    other_function();

    print_i32(12345);

    print_multiple_params(24, 'h');

    print_i32(return_math());

    println!("The tuple values are {:?}", return_tuple(12345, 1.2345, "test"));

    println!("The sum of 2 and 4 is {0}", add_or_multiply(2, 4, false));

    println!("The product of 2 and 4 is {0}", add_or_multiply(2, 4, true));

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    print_array_for(&arr);
    print_array_foreach(&arr);

    let mut vec :Vec<i32> = vec![1, 2, 3, 4, 5];
    print_array_for(vec.as_slice());
    print_array_foreach(vec.as_slice());
}

fn other_function() {
    println!("Printed from second, lesser function.");
}

fn print_i32(x: i32) {
    println!("The value of x is: {x}");
}

fn print_multiple_params(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn return_math() -> i32 {
    //6 * 6; // Wrong syntax for return, statement
    6 * 6 // Correct syntax for return, expression
}

fn return_tuple(x: i32, y: f64, z: &str) -> (i32, f64, &str) {
    (x, y, z)
}

fn add_or_multiply(x: i32, y: i32, multiply: bool) -> i32 {
    if multiply == true {
        x * y
    }
    else {
        x + y
    }
}

fn print_array_for(x: &[i32]) {
    println!("Array contains ");
    for i in 0..(x.len()) {
        println!("{number}");
    }
}

fn print_array_foreach(x: &[i32]) {
    println!("Array contains ");
    for number in x {
        println!("{number}");
    }
}
