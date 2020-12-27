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
}

fn take_ownership(some_str: String) {
    println!("some_str: {}", some_str);
} // some_str is now dropped, which means the memory backing
  // s3 is now invalid.

fn makes_copy(some_int: i32) {
    println!("some_int: {}", some_int);
}

fn gives_ownership() -> String {
    let some_str = String::from("some str");
    some_str // Ownership of some_str is passed back to the calling function
}