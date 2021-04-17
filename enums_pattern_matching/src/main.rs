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
}

fn route(ip_kind: IpAddrKind) {
    println!("Routing IP address of type {:?}", ip_kind);
}