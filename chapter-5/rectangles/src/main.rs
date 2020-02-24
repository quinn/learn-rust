#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn main() {
//     let dims = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let (dims, area) = area(dims);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area
//     );

//     println!("{:?}", dims);
// }

// fn area(dims: Rectangle) -> (Rectangle, u32) {
//     let area = dims.width * dims.height;
//     (dims, area)
// }

// fn main() {
//     let dims = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let area = dims.area();

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area
//     );

//     println!("{:?}", dims);
// }

impl Rectangle {
    fn area(self: &Rectangle) -> u32 {
        self.width * self.height
    }

    fn can_hold(self: &Rectangle, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("no cuz u a squarryyerr {:?}", Rectangle::square(100));
}
