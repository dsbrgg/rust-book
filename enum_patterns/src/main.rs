fn main() {
  println!("{:?}", IpAddrKind::V6);
}

#[derive(Debug)]
enum IpAddr {
  V4(String),
  V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V4(String::from("::1"));

// Example of function with the enum on its signature
// fn route(ip_type: IpAddrKind) {}
