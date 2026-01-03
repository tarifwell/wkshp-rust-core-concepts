/* --- Ownership & Moving ---
    
🔑 Key Concepts:
- Each value has *one* owner.
- When the owner goes out of scope, the value is dropped (memory freed).
- Simple "stack" values (like `i32`) are *copied*.
- Assigning a "heap" value (like `String`) *moves* ownership.
*/

fn main() {
    let x = 5; // x owns the value 5
    let y = x; // y is a copy of x (both own 5)
    println!("x: {}, y: {}", x, y); // both x and y are valid

    let s1 = String::from("hello"); // s1 owns the String "hello"
    let s2 = s1; // ownership of the String is moved to s2
    // println!("s1: {}", s1); // ERROR: s1 is no longer valid
    println!("s2: {}", s2); // s2 is valid

    let s3 = String::from("world");
    let len = calculate_length(s3); // ownership of s3 is moved to the function
    // println!("s3: {}", s3); // ERROR: s3 is no longer valid
    println!("Length of string: {}", len); // len is valid
}
#[allow(dead_code)]
fn calculate_length(s: String) -> usize {
    let len = s.len();
    println!("Consumed string length: {}", len);
    len // return len;
}
