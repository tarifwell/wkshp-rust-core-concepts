/* --- Shadowing ---

🔑 Key Concepts:
- Declares a new variable with the same name.
- Hides (shadows) the old variable.
- Is not mutation (it's a new let binding).
- Allows changing the type (mut can't do that).
- Follows standard scope rules.
*/

fn main() {
    println!("--- Mutability ---");

    let x = 5;
    println!("The value of x is: {}", x); //5
    // not allowed, consider making this binding mutable
    // x = 6;
    // println!("The value of x is: {}", x); // error: cannot assign twice to immutable variable, because x is not mutable

    let mut y = 7;
    println!("The value of y is: {}", y); //7
    y = 8;
    println!("The value of y is: {}", y); //8

    println!("\n--- Shadowing ---");

    let z = 5;
    println!("The value of z is: {}", z); //5

    let z = z + 1;
    println!("z is shadowed, the new value is: {}", z); //6

    {
        // This shadow only exists inside this scope.
        let z = z * 2; //12
        println!("In the inner scope, z is: {}", z);
    }

    println!("Back in the outer scope, z is: {}", z); //6

    println!("\n--- Shadowing vs. Mutability ---");

    // Shadowing can change the type of the binding.
    let spaces: &str = "   ";
    println!("'{}' is a string", spaces); // spaces is type &str
    let spaces: usize = spaces.len(); // `spaces` is now type: usize
    println!("...and now {} is a number (its length).", spaces);
}
