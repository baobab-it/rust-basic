 /*

Конкатенація рядків

*/

fn main() {
    by_moving(); // Віддаємо перевагу цьому способу, використовує менше памяті, діє прямолінійно
    by_cloning();// коли потрібно залишити дані недоторканими, проте може використовувати багато памяті
    by_mutating(); // варіант, який продуктивно працює як в першому випадку, але мутує змінну. Використовуємо у випадку великої потреби мутації даних чи в представлені стану в дуже малому і керованому контексті.
    let hello_world = "Hello world!".to_string();
    println!("index first word {}", first_word(&hello_world)); // виводимо перше слово
}

fn by_moving() {
    let hello = "hello ".to_string(); // :String
    let world = "world!";             // :&str

    // Переміщуємо hello до нової змінної
    let hello_world = hello + world;
    // Hello НЕ МОЖЕ більше використовуватись
    //println!("{}", hello);// panic
    println!("{}", world);
    println!("{}", hello_world); // -> "hello world!"
}

fn by_cloning() {
    let hello = "hello ".to_string(); // :String
    let world = "world!";             // :&str

    // Створюємо копію hello і переміщуємо в нову змінну
    let hello_world = hello.clone() + world;
    // Hello може далі використовуватись
    println!("{}", hello_world); // Друкуємо "hello world!"
}

fn by_mutating() {
    let mut hello = "hello ".to_string();
    let world = "world!";

    // Модифікуємо hello
    hello.push_str(world);
    // hello містить два рядка
    println!("{}", world);
    println!("{}", hello); // Друкуємо "hello world!"
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // отримуємо масив байтів (УВАГА використовуємо тільки для Ascii кодувань)

    for (i, &item) in bytes.iter().enumerate() { // enumerate обгортає ітератор і викодить кортеж (index, &item)
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
