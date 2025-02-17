// Calculator Program

// Function to add 2 numbers
fn add_numbers(x: i32, y: i32) -> i32 {
    return x + y;
}

// Function to subtract 2 numbers
fn subtract_numbers(x: i32, y: i32) -> i32 {
    if x > y {
        return x - y;
    }
    else {
        return y - x;
    }
}

// Function to multiply 2 numbers
fn multiply_numbers(x: i32, y: i32) -> i32 {
    return x * y;
}

// Function to divide 2 numbers
fn divide_numbers(x: i32, y: i32) -> i32 {
    return x / y;
}

fn main() {
    let a = 8;
    let b = 4;

    // Addition
    let sum = add_numbers(a, b);
    println!("Sum of {} and {} = {}", a, b, sum);

    // Subtraction
    let sum = subtract_numbers(a, b);
    if a > b { println!("Difference of {} and {} = {}", a, b, sum); }
    else { println!("Difference of {} and {} = {}", b, a, sum); }

    // Multiplication
    let sum = multiply_numbers(a, b);
    println!("Product of {} and {} = {}", a, b, sum);

    // Division
    let sum = divide_numbers(a, b);
    println!("Division of {} and {} = {}", a, b, sum);
}
