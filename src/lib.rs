mod back_of_house;
mod front_of_house;

use crate::back_of_house::{Appetizer, Breakfast};
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    println!("{}", meal.toast);

    meal.toast = String::from("Wheat");

    println!("{}", meal.toast);

    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup;
}
