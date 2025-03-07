use std::io;
fn main() {
    let mut width_input = String::new();
    let mut height_input = String::new();
    let width: u32;
    let height: u32;

    println!("insert width");
    io::stdin()
        .read_line(&mut width_input)
        .expect("faild width");

    println!("insert height");
    io::stdin()
        .read_line(&mut height_input)
        .expect("faild height");

    width = match width_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid width");
            return;
        }
    };

    height = match height_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("please enter valid height");
            return;
        }
    };

    println!("This area of rectangle is {} pixels", area(width, height));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
