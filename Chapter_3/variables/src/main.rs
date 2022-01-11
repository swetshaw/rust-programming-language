fn main() {
    // Variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // SHADOWING
    let m = 5;
    let m = m + 1;
    {
        let m = m * 2;
        println!("The value of m in the inner scope is : {}", m);
    }

    println!("The value of m is {}", m);
}
