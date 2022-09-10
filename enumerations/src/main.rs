#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i8, i8, i8)
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
    }
}

fn main() {
    let home = IpAddrKind::V4(127,0,0,1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("{:#?}, {:#?}", home, loopback);

    let m = Message::Write(String::from("heyyyy there"));
    m.call();

    let x = Some(5);
}
