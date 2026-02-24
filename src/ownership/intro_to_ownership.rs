// Ownership
/*
Ownership is a set of rules that govern how a Rust program manages memory.
All programs have to manage the way they use a computer’s memory while running.
 Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs;
 in other languages, the programmer must explicitly allocate and free the memory.
 Rust uses a third approach: Memory is managed through a system of ownership with a set of rules that the compiler checks.

 The Stack and Heap concept:
 1. In Rust whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions
 2. Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways.
 3. Stack - LIFO
    >. Adding or removing plates from the middle or bottom wouldn’t work as well! Adding data is called pushing onto the stack, and removing data is called popping off the stack.
    >. All data stored on the stack must have a known, fixed size.
 6. Heap:
    >. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
    >. Heap is less organized
    >. When you put data on heap, you must request certain amount of space.
    >. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
 7. Pushing to Heap and Stack
    >. Pushing to stack is faster then the heap, because the allocator never has to search for a place to store new data; that location is always at the top of the stack.
    >. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
 8. Accessing Data on Heap and Stack
    >. Accessing data in the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there.
    >. By the same token, a processor can usually do its job better if it works on data that’s close to other data (as it is on the stack) rather than farther away (as it can be on the heap).


Ownership Rules
First, let’s take a look at the ownership rules. Keep these rules in mind as we work through the examples that illustrate them:

  >.  Each value in Rust has an owner.
  >.  There can only be one owner at a time.
  >.  When the owner goes out of scope, the value will be dropped.

*/

pub fn intro_to_ownership() {
    basic_ownership();
}
// At the start of these examples, we will look into ownership of some variables
fn basic_ownership() {
    // Scope example
    let s = "hello";

    // This value is valid until the scope of this block

    // Let's have a block
    {
        let s1 = "hello";
        // Scope lies only in this block
    }
    // Try to access it outside
    //println!("{:?}", s1); //Cannot find value `s1` in this scope
    println!("{:?}", s); // Prints hello

    intro_to_string();
}
//println!("{:?}", s); // Cannot find value `s` in this scope

// Now use this analogy to build the concept of ownership on strings
// Because to illustrate the rules of ownership, we need a data type that is more complex than those we covered in the “Data Types”.
// Those data types are of a known size and can be stored on the stack, can be popped off, or copied to another to make a new independent instance.

// But we want to look on more complex data type stored on heap not on stack, that is string.
// String literals are convenient, but they aren’t suitable for every situation in which we may want to use text.
// You can create a String from a string literal using the from function, like so:
fn intro_to_string() {
    // Create a string from variable
    let s = String::from("hello");
    // The double colon :: operator allows us to namespace this particular from function under the String type

    // This kind of string can be mutated
    let mut s = String::from("hello");
    s.push_str(",world!"); // push_str() appends a literal to a String
    println!("{}", s); // hello,world!

    /*
    With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with our String.
    >. When we call String::from, its implementation requests the memory it needs.
    >. In Languages with a Garbage Collector G.C, GC keeps track of and cleans up memory, that is not being used.
    >. In most of languages where as it is our responsibility to keep track explicitly, which value is not in use.
    >. Rust takes different path: The memory is automatically, once the variable that owns it goes out of scope.
    */

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
    // no longer valid

    // Rust calls a special function for us. This function is called drop, and it’s where the author of String can put the code to return the memory. Rust calls drop automatically at the closing curly bracket.

    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    // Here it did not erase x when copied to y because x and y are integer values with a fixed size and pushed on stack

    // lets look into string version
    let s1 = String::from("hello");
    let s2 = s1; // Value moved here
    // println!("s1: {}, s2: {}", s1, s2); // Value used after being moved - compiler error

    /*
    Lets look how string works:
    1. String is made up of three parts : This group of data is stored on the stack
        1. Ptr -> Pointer to content on heap
        2. Len -> The length is how much memory, in bytes, the contents of the String are currently using
        3. capacity -> The capacity is the total amount of memory, in bytes, that the String has received from the allocator.
    2. String content on heap:
        1. index -> 0,1,2,3,4 => The index of values on heap
        2. value -> 'h','e','l','l','o'

    Note:: When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack.

    S1 on stack:
        Ptr         |       0xgdfgsg
        Len         |       01001000
        Capacity    |       01010100

    So this statement -> s2 = s1 copied the stack data to s2.

    S2 on stack (After copy):
        Ptr         |       0xgdfgsg
        Len         |       01001000
        Capacity    |       01010100

    But the Pointer is pointing to the same address of memory for both.
    That's why after copy it dropped the s1 and said you can use the s2 now.

    IMP Note: In many languages such as Java and C++ you have heard making a shallow copy. The upper process is a shallow copy.
    Stack data copied but heap data remained same.
    But rust called it move.
    */

    // Re-assignment of value to variable
    let mut s = String::from("hello");
    s = String::from("hyo");
    println!("s: {}", s);
    // Here initially declared variable s and bind it to a String with the value "hello".
    // Then, we immediately create a new String with the value "ahoy" and assign it to s.
    // So the value "hello" became redundant after assigning new value to s and was dropped off from heap.

    // Deep Copy (Not just stack data but also the heap data
    // We need to use the clone
    let s3 = s.clone();
    println!("s3: {}", s3); // s3: hyo
    // Heap data does copied

    // Functions
    println!("---------Function Ownership ------------");
    takes_ownership(s3);
    println!("takes_ownership() end");
    //println!("s3: {}", s3); // Value used after being moved
    let mut s4 = String::from("hello");
    s4 = return_ownership(s4);
    println!("return_ownership() end {}: {}", s4, s4); // return_ownership() end hello: hello

    // Tuple return
    let (size, string) = return_ownership_tup(s4);
    println!("return_ownership_tup() end {}: {}", string, size); // return_ownership_tup() end hello: 5

    // But let's say I need to use same data without returning same string from function we can use the reference

    // This concept is called references and borrowing
    with_reference(&string);
    println!("string with reference end::{}", string);
    /*
    Ownership value in function with ref: hello
    string with reference end::hello
    */
    mutable_reference();

    // Dangling Pointer
   // let reference_to_nothing = dangle();
}

// Ownership and Functions
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
// If we want ownership back there where we just passed from we must need to return back the value

fn return_ownership(some_string: String) -> String {
    println!("Ownership value in function: {}", some_string);
    some_string
}
// We can also return tuples
fn return_ownership_tup(some_string: String) -> (usize, String) {
    println!("Ownership value in function tup: {}", some_string);
    let len = some_string.len();
    (len, some_string)
}

// Pass reference of string
fn with_reference(some_string: &String) {
    println!("Ownership value in function with ref: {}", some_string);
}

fn mutable_reference() {
    let mut s = String::from("hello");

    change(&mut s);

    /*
    Mutable references have one big restriction: If you have a mutable reference to a value, you can have no other references to that value.
    This code that attempts to create two mutable references to s will fail:
    */

    let mut s = String::from("hello");

    let r1 = &mut s;
   // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time [E0499] second mutable borrow occurs here

    //println!("{r1}, {r2}");

    let mut s = String::from("hello");
    // As always, we can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("after push {}", some_string);
    some_string.pop();
    println!("after pop {}", some_string);
    /*
    after push hello, world
    after pop hello, worl
    */


}

// Dangling References
/*
    A pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory.
    In Rust, by contrast, the compiler guarantees that references will never be dangling references:
    If you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
*/

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }
// Error: this function's return type contains a borrowed value, but there is no value for it to be borrowed from.

/*
What happened above:
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, so its memory goes away.
  // Danger!
*/

// But if we remove & from s it works fine because Ownership is moved out, and nothing is deallocated.


/*
Note:

    The Rules of References
    Let’s recap what we’ve discussed about references:

    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.
*/