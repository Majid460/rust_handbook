// The ability to run some code depending on whether a condition is true and the ability to run some
// code repeatedly while a condition is true are basic building blocks in most programming languages.
// The most common constructs that let you control the flow of execution of Rust code are if expressions and loops.

// 1. if Expressions

use std::io;

pub fn control_flow() {
    println!("-----------Control Flow-------------");
    // let mut input: String = String::new();
    // println!("Please input your Number::");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // control_flow_example(input.trim().parse::<i32>().unwrap());
    //
    // // Using if in a let Statement
    // let x = if input.trim().parse::<i32>().unwrap() == 5 { 5 } else { 0 };
    // println!("The value of x is {}", x);
    println!("------------Loops------------------");
    loops();
}

fn control_flow_example(x: i32) {
    if x > 0 {
        println!("x > 0::x is {}", x);
    } else if x < 0 {
        println!("x < 0 :: x is {}", x);
    }
}
fn loops() {
    /*
    Rust has three kinds of loops: loop, while, and for. Let’s try each one.
    */
    // Repeating Code with loop
    // loop {
    //     println!("again!");
    // }

    // Returning from loop
    let mut counter = 0;
    let ret: i32 = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop return: {}", ret);

    println!("-----------Loop labels---------------");
    let mut counter = 0;
    'counting_loop: loop {
        println!("loop counter: {}", counter);
        let mut remaining = 10;
        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        counter += 1;
    }
    // While loops
    while counter != 0 {
        println!(" while loop counter: {}", counter);
        counter -= 1;
    }
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("-----for loop ---------");
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Range
    for number in (0..a.len()){
        println!("{}", number);
    }
    // Index with number
    for (index,num) in a.iter().enumerate(){
        println!("the value is: {num} at index {}", index);
    }

}
