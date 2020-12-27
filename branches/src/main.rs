fn main() {
    let number = 7;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    // If can be used to assign things, like ternary operator in Java
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("Number is now {}", number);

    // Loops can return things
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("The result was {}", result);

    // While loops!
    counter = 10;
    while counter != 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("LIFTOFF!");

    // For loops can iterate over collections
    let array = [3,6,8,0];
    for num in array.iter() {
        println!("The value is {}", num);
    }

    // For loops can also run for a specific range of numbers
    for num in 1..4 {
        println!("For loop {}", num);
    }
}
