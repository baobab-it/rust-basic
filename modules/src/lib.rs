mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    #[allow(dead_code)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
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
    hosting::add_to_waitlist();
    // Замовлення сніданку з тостами Rye
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Змінємо нашу думку і замоляємо хліб, який ми любимо
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Наступна лінія коду не буде компліюватись, якщо ми 
    // приберемо коментар; оскільки нам не дозволяється бачити
    // чи модифікувати сезонні фрукти, що подаються з їжею
    //meal.seasonal_fruit = String::from("blueberries");
}