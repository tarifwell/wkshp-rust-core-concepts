/*  --- Functions ---

🔑 Key Concepts
- Defined using the `fn` keyword.:
- Have a name and can take parameters.
- Parameters must have their types declared.
- Can return a value; return type is declared after `->`.
- Use `return` for early returns; otherwise, the last expression is returned.
- Distinguish between statements (do not return values) and expressions (return values).
- Functions must be defined before they are called.
*/

// function with multiple parameters and without return.
fn function_without_return(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Functions with Return. Works but not idiomatic.
fn function_with_return(num: i32) -> i32 {
    return num + 5;
}

// Functions with Return. Alternative way without `return` keyword. Recommended style in Rust.
fn function_with_implicit_return(num: i32) -> i32 {
    num + 5
}

// Function with Return, using a block expression.
fn function_with_block_expression(num: i32) -> i32 {
    let result = {
        let intermediate = num + 2;
        intermediate * 3
    };
    result
}

// Function with early return.
fn function_with_early_return(num: i32) -> i32 {
    if num > 10 {
        return num;
    }
    num + 5
}

// The `main` Function is the entry point.
fn main() {
    println!("Hello from the main function!");

    // Calling Functions
    function_without_return(10, 'm');
    let result1 = function_with_return(10);
    println!("Result from function_with_return: {}", result1);
    let result2 = function_with_implicit_return(10);
    println!("Result from function_with_implicit_return: {}", result2);
    let result3 = function_with_block_expression(4);
    println!("Result from function_with_block_expression: {}", result3);
    let result4 = function_with_early_return(12);
    println!("Result from function_with_early_return: {}", result4);
    let result5 = function_with_early_return(8);
    println!("Result from function_with_early_return: {}", result5);

    // Calling a function defined after main
    // but, in Rust, it must be declared before use
    another_function();
}

// defining functions after main is also valid in Rust
fn another_function() {
    println!("Hello from another_function!");
}
