/* --- Primitive Types: Arrays ---

🔑 Key Concepts: 
- Groups multiple values of the same type.
- Have fixed length (size cannot change).
- Type is annotated as `[type; length]`, e.g., `[i32; 5]`.
- Access elements by index with `[]` (e.g., `arr[0]`).
- Data is allocated on the stack.
*/
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array 'a' length: {}", a.len()); // 5
    println!("First element of 'a': {}", a[0]); // 1
    println!("Contents of array 'a': {:?}", a); // [1, 2, 3, 4, 5]

    let b = [3; 5]; // `[value; length]` is the same as `let b = [3, 3, 3, 3, 3];`
    println!("First and last elements of 'b': {} - {}", a[0], b[4]); // 3 - 3

    let c: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    println!("Contents of array 'c': {:?}", c); // ['a', 'b', 'c', 'd', 'e']

    // We access array elements by index (starting from 0).
    let first = a[0];
    let second = a[1];
    println!("Array 'a' elements: first = {}, second = {}", first, second); // 1, 2

    let d = [10, 20, 30]; // Type and length inferred as `[i32; 3]`
    println!("Contents of array 'd': {:?}", d); // [10, 20, 30]

    // The following line would cause a compile-time error
    //let e: [i32; 3] = [10, "a", false]; // error: mismatched types
    //let panic = a[10]; // error: index out of bounds (a runtime error)
}
