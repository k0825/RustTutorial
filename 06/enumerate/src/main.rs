enum IpAddrKind {
    v4,
    v6,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: v6,
        address: String::from("::1"),
    };

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);

    let m = Message::Write(String::from("Hello"));
    m.call()
}

fn route(ip_type: IpAddrKind) {}
