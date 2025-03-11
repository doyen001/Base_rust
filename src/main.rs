use std::io;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // meal.toast = String::from("wheat");
    println!("{}", meal.toast);
}

pub fn main() {
    crate::eat_at_restaurant();
}
// fn area(react1: Reactangle) -> u32 {
//     width * height
// }
