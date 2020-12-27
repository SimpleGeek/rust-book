fn main() {
    // A mutable variable - variables are immutable by default in Rust
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Here's an immutable constant!
    const MAX_POINTS: u32 = 100_000;
    println!("Max points are: {}", MAX_POINTS);

    // Here's shadowing!
    // It's a way to transform an immutable variable.
    // Also great for converting data types while using
    // the same identifier.
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // This is a tuple
    let tup: (i32, f64, bool, char) = (-10, 3.14, false, 'A');

    // Getting the values out of a tuple by destructuring into
    // individual variables with type-inference
    let (thing1, thing2, thing3, thing4) = tup;
    println!("Thing 1: {}", thing1);
    println!("Thing 2: {}", thing2);
    println!("Thing 3: {}", thing3);
    println!("Thing 4: {}", thing4);

    // Alternatively, retrieve values by using indexes in the array
    println!("Thing 4: {}", tup.3);
    println!("Thing 3: {}", tup.2);
    println!("Thing 2: {}", tup.1);
    println!("Thing 1: {}", tup.0);

    // An array!
    let the_array = [1,2,3];


    // An array with specified types and length
    let the_array2: [i32; 2] = [4, 5];

    // An array with 5 "hellos" in it.
    // This is equivalent to ["hello", "hello", ...]
    let the_array3 = ["hello"; 5];

    // Retrieving an element using the index
    let hello = the_array3[0];
}
