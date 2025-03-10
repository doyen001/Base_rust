use std::io;

struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let react1 = Reactangle {
        width: 30,
        height: 20,
    };

    println!("This area of rectangle is {} pixels", react1.area());
}

// fn area(react1: Reactangle) -> u32 {
//     width * height
// }
