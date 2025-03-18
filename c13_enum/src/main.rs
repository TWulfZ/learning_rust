// Enums

use std::net::IpAddr;

fn main() {
    // enum IpAddrKind {
    //     V4,
    //     V6
    // }
    // let four = IpAddr::V4;
    // let six = IpAddr::V6;

    // fn route(ip_kind: IpAddrKind) {
        
    // }

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    // // Using structs
    // enum IpAddrKind {
    //     V4(String),
    //     V6(String)
    // }

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // Using structs
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
}

