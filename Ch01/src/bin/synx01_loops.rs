
fn main() {
    println!(">>> Приклад роботи з циклу loop");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Результат {}", result);

    println!("=======");

    println!(">>> Приклад роботи з циклу while");

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("=======");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("значення: {}", a[index]);

        index += 1;
    }

    println!("=======");

    println!(">>> Приклад роботи з циклу for");

    for element in a.iter() {
        println!("значення: {}", element);
    }

    println!("=======");

    println!(">>> Приклад роботи з циклу for в зворотньому порядку");

    for element in a.iter().rev() {
        println!("значення: {}", element);
    }

    println!("=======");

    println!(">>> Приклад роботи з циклу for по згенерованому діапазону в зворотньому порядку");

    for number in (1..4).rev() {
        println!("значення: {}!", number);
    }
}