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
}