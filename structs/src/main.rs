 /*
 Code from following rust book section 5.1
  */
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// This struct would cause a compile error, since
// structure fields can't be declared as reference
// (unowned) types without lifetimes.
/*
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
 */

fn not_main() {
    println!("Hello, world!");

    // Struct instances are immutable by default,
    // must be made mutable.  Only entire instance
    // can be made mutable, there is no control over
    // individual fields.
    let mut user1 = User {
        email: String::from("hello@world.com"),
        username: String::from("hellow0rld"),
        active: true,
        sign_in_count: 1
    };

    // Set properties with dot notation
    user1.email = String::from("anotheremail@world.com");

    // Demonstration of update syntax.  The first two
    // variables for this new struct instance are specified,
    // the rest are taken from the user1 instance with the
    // ".." syntax.
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("McAnother"),
        ..user1
    };

    // Struct tuples can be created by creating structures
    // with types but no field names.
    struct Color(i32, i32, i32);
    let white = Color(255, 255, 255);
    let red_val = white.0;
}

fn build_user(email: String, username: String) -> User {
    // When the variables you are assigning to the fields
    // in a structure have the same names as the fields in
    // said structure, you can simply list the name as
    // assignment shorthand.
    User {
        email,
        username,
        active: true,
        sign_in_count: 10,
    }
}

 /*
 Code from following rust book section 5.2
  */
 struct Rectangle {
     width: u32,
     height: u32,
 }

 fn main() {
     let rect1 = Rectangle {
         width: 35,
         height: 50,
     };

     println!(
        "The area of the rectangle is {} square pixels.",
         area(&rect1)
     );
 }

 fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
 }