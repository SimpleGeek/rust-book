/*
    Notes about statements and expressions in Rust:
    A statement is code that does a thing, and does not
    return a value.  An expression is code that evaluates to
    a value.  This could be a literal, or a match equation, for example.
    If you append a semicolon to an expression, it becomes a statement,
    and will no longer return a value.
 */

fn main() {
    another_function(-5, 10);

    let x = five();
    println!("x in main is: {}", x);

    let x2 = plus_one(x);
    println!("x2 in main is: {}", x2);
}

fn another_function(x: i32, y: u32) {
    println!("x: {}", x);
    println!("y: {}", y);
}

// This function contains an expression, as the literal
// value five, which is returned as an i32.  You can return
// early from a function using the return keyword, but the
// Rust standard is to implicitly return the final expression
// in the function body.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
