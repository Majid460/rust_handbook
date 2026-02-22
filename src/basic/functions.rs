
/*
    Functions
    1. Uses fn keyword
    2. Name must be in snake case -> name_snake
    3. Set of parentheses () and body starts with {}

Note: If you add a semicolon to the end of an expression,
      you turn it into a statement, and it will then not return a value.
      Keep this in mind as you explore function return values and expressions next.



*/
pub fn functions(){
    println!("-------------------Functions----------------");
    let mut  x = 1;
    another_function(x);
    x = return_function(x);
    println!("The value of x is: {}", x);
}
fn another_function(x:i32){
    println!("Value of x is {}", x);
}
/*
Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->).
*/
fn return_function(x:i32)->i32{
    x+5
}