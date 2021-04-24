#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Enums can also be used in structs
struct IpAddress {
    kind: IpAddrKind,
    addr: String,
}

// Enum variants can have associated values
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Enum values can be assigned like this
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddress {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let new_home = IpAddr::V4(127, 0, 0, 1);

    // We can use the Option enum to account for missing values
    let not_here: Option<u8> = None;
    println!("Is not here, well, here? {}", not_here.is_some());

    println!("Value in cents of coin type {:?} is {}", Coin::Nickel, value_in_cents(Coin::Nickel));
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing IP address of type {:?}", ip_kind);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Arkansas,
    Alaska,
    Hawaii,
    North_Carolina,
    South_Carolina,
    West_Virginia,
    Vermont,
    Virginia,
    // etc
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}