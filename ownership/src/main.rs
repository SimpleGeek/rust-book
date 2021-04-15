fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // This works because ints are stored on the stack, so variable assignments are really deep copies.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("AString");
    //let s2 = s1;  In this example, s2 now "owns" the value in s1, so s1 can't be used after this point
    let s2 = s1.clone(); // This is a deep copy of the data in s1 to s2, so both can be used
    println!("{}", s1);

    let s3 = String::from("hejjo");
    take_ownership(s3);
    // Cannot use s3 anymore - it was dropped by take_ownership

    let z = 19;
    makes_copy(z);
    // z can still be used because it is stored on the stack,
    // which means makes_copy just made a copy of the value.

    // s4 now belongs to main and will go out of scope once main is complete
    let s4 = gives_ownership();

    let s5 = String::from("string#5");
    // Here, we pass a reference to let the function borrow the value
    let len_of_s5 = calculate_length(&s5);

    let mut to_be_duded = String::from("Random string");
    change(&mut to_be_duded);

    let s6 = String::from("Couple of words here");
    println!("First word in {} is {}", s6, first_word(&s6));
}

fn take_ownership(some_str: String) {
    println!("some_str: {}", some_str);
} // some_str is now dropped, which means the memory backing
  // s3 is now invalid.

// Makes a copy because this is data stored on the stack
fn makes_copy(some_int: i32) {
    println!("some_int: {}", some_int);
}

fn gives_ownership() -> String {
    let some_str = String::from("some str");
    some_str // Ownership of some_str is passed back to the calling function
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string - called borrowing
    s.len() // s can't be mutated here, because no ownership
} // Here, the scope of s ends, but nothing happens since this is a reference

// Specifying &mut allows us to mutate the argument reference
fn change(some_string: &mut String) {
    // Note that there can only be one variable assigned
    // to a mutable reference within a specific scope.
    // One-to-one.
    // Also, you can't have simultaneous mutable and immutable
    // references to the same object.
    some_string.push_str(", dude?");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Returning a slice of the string from
            // index 0 to i.
            return &s[0..i];
        }
    }

    &s[..]
}