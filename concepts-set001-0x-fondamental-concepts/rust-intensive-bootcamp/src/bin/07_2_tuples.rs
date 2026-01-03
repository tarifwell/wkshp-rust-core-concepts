/* --- Primitive Types: Tuples ---

🔑 Key Concepts: 
- Groups multiple values of different types.
- Have a fixed length (size cannot change).
- Access elements by index with a `.` (e.g., `tup.0`, `tup.1`).
- Can be "destructured" to get all values at once.
*/
fn main() {
    let newtup: (i32, char, bool) = (42, 'R', false);
    println!("New Tuple: {:?}", newtup);

    // Destructuring the new tuple
    let (a, b, c) = newtup;
    println!("Destructured New Tuple: a = {}, b = {}, c = {}", a, b, c);

    // Accessing elements by index
    let num = newtup.0;
    let letter = newtup.1;
    let flag = newtup.2;
    println!("Accessed by index: {}, {}, {}", num, letter, flag);
}
