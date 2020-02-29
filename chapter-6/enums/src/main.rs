
// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("{:?}", self);
//     }
// }

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// fn main() {
//     let some_u8_value = Some(3u8);

//     println!("{:?}", some_u8_value);

//     match some_u8_value {
//         Some(3) => println!("three"),
//         _ => println!("wat"),
//     }
// }

fn main() {
    let value = Some(3u8);

    if value == Some(3) {
        println!("regs equal works");
    }

    if let Some(3) = value {
        println!("WTF is the point of this.");
    }

    if let Some(x) = value {
        println!("Oh I get it: {}", x);
    }
}
