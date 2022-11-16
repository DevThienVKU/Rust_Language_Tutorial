enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr1 {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// ==
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


#![allow(unused)]
fn main() {
    //Enum Values
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    //another
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    
    //another
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //another
    let home = IpAddr1::V4(String::from("127.0.0.1"));

    let loopback = IpAddr1::V6(String::from("::1"));

    //another
    let home = IpAddr2::V4(127, 0, 0, 1);

    let loopback = IpAddr2::V6(String::from("::1"));

    //another solution
    struct Ipv4Addr {
        // --snip--
    }
    struct Ipv6Addr {
        // --snip--
    }
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    //another
    let m = Message::Write(String::from("hello"));
    m.call();

    //The Option Enum and Its Advantages Over Null Values
    enum Option<T> {
        None,
        Some(T),
    }
    
    //Examples of using Option values
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    

}
//Enum Values
fn route(ip_kind: IpAddrKind) {}

