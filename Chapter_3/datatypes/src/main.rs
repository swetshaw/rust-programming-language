fn main() {
    // addition
    let sum = 5 + 10;
    println!("The sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {}", quotient);

    let floored = 8 / 3;
    println!("The floored is: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {}", remainder);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (p, q, r) = tup;
    println!("The value of q is: {}", q);
}
