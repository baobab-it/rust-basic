/**
 * При роботі з структурами, що представляють конфігурацію, ви не турбуєтесь
 *  про деякі значення і просто хочете їм тихо призначити стандартні значення
*/
fn main() {
    // Майже для кожного примітивного типу існує значення по-замовчуванню
    let foo: i32 = Default::default();
    println!("foo: {}", foo); // -> "foo: 0"

    // Структура, яка походить від Default може бути так ініціалізована
    let pizza: PizzaConfig = Default::default();
    println!("wants_cheese: {}", pizza.wants_cheese); // -> "wants_cheese: false
    
    println!("number_of_olives: {}", pizza.number_of_olives); // -> "number_of_olives: 0"

    println!("special message: {}", pizza.special_message); // -> "special_message: "

    let crust_type = match pizza.crust_type {
        CrustType::Thin => "Nice and thin",
        CrustType::Thick => "Extra thick and extra filling",
    };
    
    println!("crust_type: {}", crust_type); // -> "crust_type: Nice and thin"

    // Ви можете також зконфігурувати тільки певні значення
    let custom_pizza = PizzaConfig {
        number_of_olives: 12,
        ..Default::default()
    };

    println!("custom_pizza: {:?}", custom_pizza); 

    // Ви можете визначити так багато значень як ви хочете
    let deluxe_custom_pizza = PizzaConfig {
        number_of_olives: 12,
        wants_cheese: true,
        special_message: "Will you marry me?".to_string(),
        ..Default::default()
    };

    println!("deluxe_custom_pizza: {:?}", deluxe_custom_pizza); // -> "custom_pizza : Nice and thin"

    let _thick = CrustType::Thick;
}

#[derive(Default,Debug)]
struct PizzaConfig {
    wants_cheese: bool,
    number_of_olives: i32,
    special_message: String,
    crust_type: CrustType,
}

// Ви можете легко реалізувати значення по-замовчуванню для ваших власних типів

#[derive(Debug)]
enum CrustType {
    Thin,
    Thick,
}
impl Default for CrustType {
    fn default() -> CrustType {
        CrustType::Thin
    }
}
