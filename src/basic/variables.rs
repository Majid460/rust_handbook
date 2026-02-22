// By default, variables are immutable. This is one of many nudges Rust gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers.
pub fn variable() {
    let x: i8 = 5; // Immutable by default
    println!("Value of x {x}");
    // x = 6; // Generate error
    println!("Value of x {x}");

    println!("----------Mutable-----------");
    // Example with mutable
    let mut y = 4;
    println!("Value of y {y}");
    y = 5;
    println!("Value of y {y}");

    println!("----------Constant ----------");
    // Always immutable and declare with const key word
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three Hours in seconds:: {} secs", THREE_HOURS_IN_SECONDS);

    println!("---------Shadowing-----------");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{} spaces", spaces);

}
