/*
Комбінування рядків, можна використовувати для обєднання з
іншими типами даних, таким як числа
*/
fn main() {
    let colour:&str = "red";
    // '{}' - це форматований сценарій, який замінюється параметром
    let favourite = format!("My favourite colour is {}", colour);
    println!("{}", favourite);

    // Ви можете додавати декілька параметрів один за одним
    let hello = "hello ";
    let world = "world!";
    let hello_world = format!("{}{}", hello, world);
    println!("{}", hello_world); // -> "hello world!"

    // Макрос format! може проводити конкатенацію будь-
    // яких типів даних, що реалізують трейт  'Display', 
    // наприклад чисел
    let favourite_num = format!("My favourite number is {}", 42);
    println!("{}", favourite_num); // -> "My favourite number is 42"

    // Якщо ви хочете включити певні параметри декілька разів
    // до рядка, ви можете скористатись параметром позиціонування
    let duck_duck_goose = format!("{0}, {0}, {0}, {1}!", "duck", "goose");
    println!("{}", duck_duck_goose); // -> "duck, duck, duck, goose!"

    // внутрішньо format! використовує підрахунок коли він стикається з {}
    println!("{}", format!("{1} {} {0} {}", "a", "b")); // -> "b a a b"

    // Ви можете навіть іменувати ваші параметри!
    let introduction = format!(
        "My name is {surname}, {forename} {surname}",
        surname = "Bond",
        forename = "James"
    );
    println!("{}", introduction); // -> "My name is Bond, James Bond"

    // важливо зазначити, що неіменновані параметри повинні зазначатись
    // першими, а вже потім іменовані
    let friendo = format!("{message} {}", "friendo", message="Hello there, ");
    println!("{}", friendo);

    // в макросі format! можна використовувати різні форматування,
    // наприклад, відобразити числа з плавоючою точкою "x.xx",
    // для цього вказуємо аргумент з числом знаків після коми
    // деталі по форматуванню https://doc.rust-lang.org/std/fmt/
    println!("{}", format!("{:.*}", 3, 1.234567)) // -> "1.235"
}
