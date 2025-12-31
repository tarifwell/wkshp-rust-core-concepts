/* Variables 

🔑 Key Concepts:
- Declared using `let`.
- Immutable by default; they cannot be changed after being assigned.
- Use the `mut` keyword to make a variable mutable.
- Rust can usually infer the type (e.g., let x = 5; is inferred as i32).
- Variables exist only within their scope (the {...} block they are in).
*/
fn main() {
    let mut x; // mutable variable
    x = 5;
    println!("The value of x is: {}", x);

    // x = "hello"; // error : expected integer, found &str
    x = 6;
    println!("The value of x is: {}", x);

    let y: i32 = 10; // immutable variable with explicit type
    println!("The value of y is: {}", y);

    // ???
    // y = y*10; // error : cannot assign twice to immutable variable `y`, make it mutable with `mut`
    // let mut y = 20;
    let mut y = y * 5;
    println!("The value of y is: {}", y);
}

// to run type the following command in your terminal:
// $ cargo run --bin 02_variables
