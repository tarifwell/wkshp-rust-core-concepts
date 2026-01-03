/* --- Ownership & Moving ---
    
🔑 Key Concepts:
- Ownership is Rust's way of managing memory. It's unique to Rust! and has deep implications for safety and performance, and how the language works.
- Each value has *one* owner.
- When the owner goes out of scope, the value is dropped (memory freed).
- Simple "stack" values (like `i32`) are *copied*.
- Assigning a "heap" value (like `String`) *moves* ownership.
*/

fn main() {
    // --- Example 1: Stack Data (Copy) ---
    // Integers are stored on the stack and implement the Copy trait.
    // Therefore, assigning one integer to another copies the value.
    // Both x and y are valid and independent.
    // Primitive types like integers, booleans, and floats implement the Copy trait, they are stored on the stack and they havent't ownership moved.
    let x = 10;
    let y = x;
    println!("x = {}", y); // 10
    println!("y = {}", x); // 10

    // --- Example 2: Heap Data (Move) --- (not common example, just for demonstration)

    // Strings are stored on the heap and do NOT implement the Copy trait.
    // Therefore, assigning one String to another MOVES ownership.
    // After the move, the original variable is no longer valid.
    // To keep both valid, we can clone the String.
    let s1 = String::from("hello");
    //let s2 = s1; // Ownership of the String MOVES from s1 to s2.
    // println!("s1 = {}", s1); // ERROR: s1 is no longer valid!
    // To keep both valid, we can clone:
    let s2 = s1.clone(); // Deep copy of the String data.
    println!("s2 = {}", s2); // "hello"
    println!("s1 = {}", s1); // "hello"

    // --- Example 3: Function Ownership Transfer --- (frequent situation)

    // Passing a String to a function MOVES ownership into the function.
    let s3 = String::from("Rust");

    // calculate_length(s3); // Ownership of s3 MOVES into the 'calculate_length' function.
    // println!("s3 = {}", s3); // Error: s3 is no longer valid!

    calculate_length(s3.clone()); // We clone s3 to keep ownership in main.
    println!("s3 = {}", s3); // "Rust"

    // This solution works, but cloning can be inefficient for large data. And cloning each time we want to use the data after passing it to a function is cumbersome.
    // In the next examples, we will explore references and borrowing to solve this problem more elegantly
}

#[allow(dead_code)]
fn calculate_length(s: String) -> usize {
    let len = s.len();
    println!("Consumed string length: {}", len); // 4
    len // return len;
}
