/* Constants 

🔑 Key Concepts:
- Declared using `const`.
- Always immutable (can't use `mut` ... no doubt if it is immutable or not).
- Type annotation is required.
- Value must be known at compile time.
- Naming convention: UPPERCASE_WITH_UNDERSCORES.
- Live for the entire program duration.
- Inlined where used (replaced with the value at compile time).
*/

const MAX_POINTS: i32 = 100_000;

/*  can't do this for constants:
    const MIN_POINTS: i32; // error : missing value for `MIN_POINTS`
    MIN_POINTS = 100; // error : missing type for `MIN_POINTS`
*/

fn main() {
    println!("A constant, MAX_POINTS, has the value: {}", MAX_POINTS);
}
