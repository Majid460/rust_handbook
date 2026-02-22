
pub fn data_types(){
    println!("--------DataTypes---------");

    // Example 1
    // let guess = "42".trim().parse().expect("Not a number!"); // It shows a error: Must consider giving `guess` an explicit type

    /*
    Rust has two data type subset: 1. Scalar 2. Compound

    1. Scalar: A scalar type represents a single value.
    Rust has four primary scalar types:
        1. integers
        2. floating-point numbers
        3. Booleans
        4. characters.
    You may recognize these from other programming languages. Let’s jump into how they work in Rust.
    */


    // 1. Integers: An integer is a number without a fractional component.
    /*
    Length	 |  Signed   |    Unsigned
    ---------|-----------|------------
    8-bit	 |    i8	 |       u8
    ---------|-----------|------------
    16-bit	 |  i16	     |       u16
    ---------|-----------|------------
    32-bit	 |  i32	     |       u32
    ---------|-----------|------------
    64-bit	 |  i64 	 |       u64
    ---------|-----------|------------
    128-bit	 |  i128	 |      u128
    ---------|-----------|------------
    Architecture- isize	 |      usize
    dependent	         |
    ----------------------------------

    1. Signed - Can have sign positive or negative
        must be: −(2^n − 1) to 2^n − 1 − 1
        i8 can store numbers from −(2^7) to 2^7 − 1 = −128 to 127
    2. Unsigned - Does not have any sign
       must be: 0 to 2^n − 1
       u8 can store numbers from 0 to 2^8 − 1
    */
    let sign:i8 = 1;
    let u_sign:u8 = 2;

    // 2. Floating-Point Types: Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.
    /*
    Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively
    All floating-point types are signed.
    The default type is f64 because on modern CPUs, it’s roughly the same speed as f32
    */

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    numeric_operations();

    // 3. Boolean: True and False
    let x:bool = true;
    let y:bool = false;

    // 4. Char
    /*
    Rust’s char type is the language’s most primitive alphabetic type. Here are some examples of declaring char values:
    */
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The char value: {}",heart_eyed_cat);

    // 2. Compound Types: Compound types can group multiple values into one type. Rust has two primitive compound types:
    /*
        1. tuples
        2. arrays

        1. Tuple:
        A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
        Tuples have a fixed length: Once declared, they cannot grow or shrink in size.
        Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    */
    let tup:(i32,bool,char) = (1,true,'a');
    println!("tup:{:?}",tup);
    println!("tup2:{}",tup.1);
    let (x, y, z) = tup; // destructuring
    println!("x:{},y:{},z:{}",x,y,z);

    // 2. Array: Another way to have a collection of multiple values is with an array.
    /*
    Difference from tuple:
    Unlike a tuple, every element of an array must have the same type.
    Unlike arrays in some other languages, arrays in Rust have a fixed length.

    Note:
    Arrays are useful when you want your data allocated on the stack,
    An array isn’t as flexible as the vector type, though.
    A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap.
    */

    let a = [1, 2, 3, 4, 5];
    println!("a[0]: {}",a[0]); // access element based on index array[i]
    println!("array:{:?}",a);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // Initialize an array with 3's has length 5
    println!("a::: {:?}",a);
}
fn numeric_operations(){
    //addition
    let add = 5+4;
    //subtract
    let subtract = 5-4;
    //multiply
    let multiplication = 5*4;

    //division
    let division = 5/4;
    //quotient
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 5%4;
}